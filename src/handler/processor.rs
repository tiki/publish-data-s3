/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::handler::{
    client::Client, config::Config, file_utils::batch::Batch, file_utils::decompress::Decompress,
};
use apache_avro::Schema;
use bytes::Bytes;
use futures::{future, FutureExt};
use tiki_private_ingest::schema::generic::{avro_schema, Record};
use tokio::io::AsyncBufReadExt;
use tokio_util::io::StreamReader;
use uuid::Uuid;

pub struct Processor {
    pub config: Option<Config>,
    pub client: Option<Client>,
    pub schema: Option<Schema>,
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

    pub fn with_schema(mut self, schema: Schema) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn auto_schema(self) -> Self {
        self.with_schema(avro_schema())
    }

    pub async fn process(
        &self,
        mut stream: StreamReader<aws_sdk_s3::primitives::ByteStream, Bytes>,
    ) {
        let config = self.config.as_ref().expect("Set Config before calling process");
        let schema = self.schema.as_ref().expect("Set Schema before calling process");
        let client = self.client.as_ref().expect("Set Client before calling process");
        stream
            .from(config.compression)
            .lines()
            .process(10000, config.file_type, |records: Vec<Record>| {
                let mut avro = apache_avro::Writer::with_codec(
                    &schema,
                    Vec::new(),
                    apache_avro::Codec::Snappy,
                );
                for record in records {
                    match avro.append_ser(record) {
                        Ok(_) => { /*do nothing*/ }
                        Err(error) => {
                            tracing::warn!(?error, "Avro serialization failed. Skipping.");
                        }
                    }
                }
                match avro.into_inner() {
                    Ok(binary) => {
                        let key = format!("{}/{}.avro", config.table, Uuid::new_v4());
                        Box::pin(
                            client
                                .write(config.bucket.to_string(), key, binary)
                                .then(|res| match res {
                                    Ok(_) => future::ready(()),
                                    Err(error) => {
                                        tracing::error!(?error, "Failed to write avro");
                                        future::ready(())
                                    }
                                }),
                        )
                    }
                    Err(error) => {
                        tracing::error!(?error, "Failed to serialize avro");
                        Box::pin(future::ready(()))
                    }
                }
            })
            .await;
    }
}
