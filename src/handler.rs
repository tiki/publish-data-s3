/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod client;
mod file_utils;

use apache_avro;
use aws_lambda_events::{event::sqs::SqsEventObj, s3::S3Event};
use client::Client;
use file_utils::{batch::Batch, decompress::Decompress};
use futures::{future, FutureExt};
use lambda_runtime::{Error, LambdaEvent};
use tiki_private_ingest::schema::generic::{avro_schema, Record};
use tokio::io::AsyncBufReadExt;

pub async fn handle(event: LambdaEvent<SqsEventObj<S3Event>>) -> Result<(), Error> {
    let client = Client::new("us-east-2").await;
    let schema = avro_schema();
    for record in &event.payload.records {
        let mut stream = client
            .read(
                "mytiki-staging-attain",
                "weekly_delivery/2023/8/29/tiki_user_demo_2023-08-21.csv.gz",
            )
            .await
            .unwrap();

        stream
            .from("gzip")
            .lines()
            .process(10000, "csv", |records: Vec<Record>| {
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
                    Ok(binary) => Box::pin(client.write("", "", binary).then(|res| match res {
                        Ok(_) => future::ready(()),
                        Err(error) => {
                            tracing::error!(?error, "Failed to write avro");
                            future::ready(())
                        }
                    })),
                    Err(error) => {
                        tracing::error!(?error, "Failed to serialize avro");
                        Box::pin(future::ready(()))
                    }
                }
            })
            .await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
