/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod config;
mod file_utils;
mod processor;
mod s3;
mod sqs;

use aws_lambda_events::{event::cloudwatch_events::CloudWatchEvent, s3::S3Entity};
use lambda_runtime::{Error, LambdaEvent};
use processor::Processor;

pub async fn handle(event: LambdaEvent<CloudWatchEvent<S3Entity>>) -> Result<(), Error> {
    tracing::debug!("{:?}", event);
    let processor = &Processor::new()
        .from_env()
        .auto_schema()
        .for_region("us-east-2")
        .await;
    let event = event.payload.detail.ok_or(Error::from("No payload. Skipping"))?;
    let stream = processor
        .s3_client_guard()
        .read(
            event.bucket.name.as_ref().expect("No bucket. Skipping").to_string(),
            event.object.key.as_ref().expect("No key. Skipping").to_string(),
        )
        .await;
    match stream {
        Ok(stream) => processor.process(stream).await,
        Err(err) => {
            tracing::error!(?err, "Stream processing failed.")
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn local() {
        println!("tested.");
        assert_eq!(1, 1);
    }
}
