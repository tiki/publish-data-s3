/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::handler::{
    config::Config, file_utils::batch::Batch, file_utils::decompress::Decompress, s3, sqs,
};
use bytes::Bytes;
use futures::{future, FutureExt};
use parquet::{
    file::{properties::WriterProperties, writer::SerializedFileWriter},
    record::RecordWriter,
    schema::types::Type,
};
use std::error::Error;
use tiki_private_ingest::schema::generic;
use tokio::io::AsyncBufReadExt;
use tokio_util::io::StreamReader;
use uuid::Uuid;

pub struct Processor {
    pub config: Option<Config>,
    pub s3_client: Option<s3::client::Client>,
    pub sqs_client: Option<sqs::client::Client>,
    pub schema: Option<Type>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            config: None,
            s3_client: None,
            sqs_client: None,
            schema: None,
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn from_env(self) -> Self {
        self.with_config(Config::from_env())
    }

    pub fn with_s3_client(mut self, client: s3::client::Client) -> Self {
        self.s3_client = Some(client);
        self
    }

    pub fn with_sqs_client(mut self, client: sqs::client::Client) -> Self {
        self.sqs_client = Some(client);
        self
    }

    pub async fn for_region(self, region: &str) -> Self {
        self.with_s3_client(s3::client::Client::new(region).await)
            .with_sqs_client(sqs::client::Client::new(region).await)
    }

    pub fn with_schema(mut self, schema: Type) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn auto_schema(self) -> Self {
        self.with_schema(generic::parquet())
    }

    pub fn config_guard(&self) -> &Config {
        self.config.as_ref().expect("Config not decclared")
    }

    pub fn schema_guard(&self) -> &Type {
        self.schema.as_ref().expect("Schema not declared")
    }

    pub fn s3_client_guard(&self) -> &s3::client::Client {
        self.s3_client.as_ref().expect("S3 client not declared")
    }

    pub fn sqs_client_guard(&self) -> &sqs::client::Client {
        self.sqs_client.as_ref().expect("SQS client not declared")
    }

    fn write_parquet(&self, records: Vec<generic::Record>) -> Result<Vec<u8>, Box<dyn Error>> {
        let props = WriterProperties::builder().build();
        let mut parquet = Vec::<u8>::new();
        let schema = self.schema_guard().to_owned();
        let mut parquet_writer =
            SerializedFileWriter::new(&mut parquet, schema.into(), props.into())?;
        let mut parquet_group_writer = parquet_writer.next_row_group()?;
        (&records[..]).write_to_row_group(&mut parquet_group_writer)?;
        parquet_group_writer.close()?;
        parquet_writer.close()?;
        Ok(parquet)
    }

    async fn write_s3(&self, key: String, binary: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let config = self.config_guard();
        let client = self.s3_client_guard();
        let _ = client.write(config.bucket.to_string(), key, binary).await?;
        Ok(())
    }

    async fn write_sqs(&self, key: String, count: i64) -> Result<(), Box<dyn Error>> {
        let config = self.config_guard();
        let file_properties = self
            .s3_client_guard()
            .read_metadata(config.bucket.to_string(), key.to_string())
            .await?;
        let file_metadata = sqs::file_metadata::FileMetadata::new()
            .with_table(&config.table)
            .with_uri(&format!("s3://{}/{}", config.bucket, key.to_string()))
            .with_size(file_properties.object_size)
            .with_count(count);
        let _ = self
            .sqs_client_guard()
            .write(&config.queue, &file_metadata)
            .await?;
        Ok(())
    }

    async fn write_aws(&self, count: i64, binary: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let config = self.config_guard();
        let key = format!("{}/data/{}.parquet", config.table, Uuid::new_v4());
        self.write_s3(key.to_string(), binary).await?;
        self.write_sqs(key.to_string(), count).await?;
        Ok(())
    }

    pub async fn process(
        &self,
        mut stream: StreamReader<aws_sdk_s3::primitives::ByteStream, Bytes>,
    ) {
        let config = self.config_guard();
        stream
            .from(config.compression)
            .lines()
            .process(
                config.max_rows,
                config.file_type,
                |records: Vec<generic::Record>| {
                    let record_count =
                        i64::try_from(records.len()).expect("Can't fit usize in i64. Panic!");
                    let parquet = self.write_parquet(records);
                    match parquet {
                        Ok(parquet) => {
                            Box::pin(self.write_aws(record_count, parquet).then(|res| match res {
                                Ok(_) => future::ready(()),
                                Err(err) => {
                                    tracing::error!(?err, "Failed to write to aws.");
                                    future::ready(())
                                }
                            }))
                        }
                        Err(err) => {
                            tracing::error!(?err, "Failed to write to parquet.");
                            Box::pin(future::ready(()))
                        }
                    }
                },
            )
            .await;
    }
}
