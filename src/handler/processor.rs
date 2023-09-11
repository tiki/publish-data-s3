/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::handler::{
    client::Client, config::Config, file_utils::batch::Batch, file_utils::decompress::Decompress,
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
    pub client: Option<Client>,
    pub schema: Option<Type>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            config: None,
            client: None,
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

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn for_region(self, region: &str) -> Self {
        self.with_client(Client::new(region).await)
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

    pub fn client_guard(&self) -> &Client {
        self.client.as_ref().expect("Client not declared")
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

    async fn write_s3(&self, binary: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let config = self.config_guard();
        let client = self.client_guard();
        let key = format!("{}/{}.parquet", config.table, Uuid::new_v4());
        let _ = client.write(config.bucket.to_string(), key, binary).await?;
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
            .process(100000, config.file_type, |records: Vec<generic::Record>| {
                let parquet = self.write_parquet(records);
                if parquet.is_err() {
                    let err = parquet.unwrap_err();
                    tracing::error!(?err, "Failed to write to parquet.");
                    return Box::pin(future::ready(()));
                } else {
                    let res = self.write_s3(parquet.unwrap());
                    return Box::pin(res.then(|res| match res {
                        Ok(_) => future::ready(()),
                        Err(err) => {
                            tracing::error!(?err, "Failed to write to s3.");
                            future::ready(())
                        }
                    }));
                }
            })
            .await;
    }
}
