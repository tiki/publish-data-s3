/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod client;
mod config;
mod file_utils;
mod processor;

use aws_lambda_events::{event::sqs::SqsEventObj, s3::S3Event};
use lambda_runtime::{Error, LambdaEvent};
use processor::Processor;

pub async fn handle(event: LambdaEvent<SqsEventObj<S3Event>>) -> Result<(), Error> {
    let processor = &Processor::new()
        .from_env()
        .auto_schema()
        .for_region("us-east-2")
        .await;

    for record in &event.payload.records {
        for s3_record in &record.body.records {
            if s3_record.s3.bucket.name.is_none() || s3_record.s3.object.key.is_none() {
                tracing::error!("Invalid payload - no s3 bucket or key. Skipping");
            } else {
                let stream = processor
                    .client
                    .as_ref()
                    .expect("Client not initialized.")
                    .read(
                        s3_record.s3.bucket.name.as_ref().unwrap().to_string(),
                        s3_record.s3.object.key.as_ref().unwrap().to_string(),
                    )
                    .await;
                if let Ok(stream) = stream {
                    processor.process(stream).await;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
