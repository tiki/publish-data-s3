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
                    .client_guard()
                    .read(
                        s3_record.s3.bucket.name.as_ref().unwrap().to_string(),
                        s3_record.s3.object.key.as_ref().unwrap().to_string(),
                    )
                    .await;
                match stream {
                    Ok(stream) => processor.process(stream).await,
                    Err(err) => {
                        tracing::error!(?err, "Stream processing failed.")
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::handler;
    use aws_lambda_events::s3::{
        S3Bucket, S3Entity, S3Event, S3EventRecord, S3Object, S3RequestParameters, S3UserIdentity,
    };
    use aws_lambda_events::sqs::{SqsEventObj, SqsMessageObj};
    use chrono::DateTime;
    use lambda_runtime::{Context, LambdaEvent};
    use std::collections::HashMap;
    use std::env;
    use uuid::Uuid;

    #[tokio::test]
    async fn local() {
        env::set_var("TIKI_BUCKET", "mytiki-test");
        env::set_var("TIKI_TABLE", "");
        env::set_var("TIKI_FILE_TYPE", "csv");
        env::set_var("TIKI_COMPRESSION", "gzip");

        let mut context = Context::default();
        context.request_id = Uuid::new_v4().to_string();

        let mut payload = SqsEventObj::<S3Event>::default();
        let mut records = Vec::<SqsMessageObj<S3Event>>::new();
        let mut record = SqsMessageObj::<S3Event>::default();

        let mut s3_event = S3Event::default();
        let mut s3_event_records = Vec::<S3EventRecord>::new();
        let s3_event_record = S3EventRecord {
            event_version: None,
            event_source: None,
            aws_region: None,
            event_time: DateTime::default(),
            event_name: None,
            principal_id: S3UserIdentity { principal_id: None },
            request_parameters: S3RequestParameters {
                source_ip_address: None,
            },
            response_elements: HashMap::new(),
            s3: S3Entity {
                bucket: S3Bucket {
                    arn: None,
                    name: Some(String::from("")),
                    owner_identity: None,
                },
                configuration_id: None,
                schema_version: None,
                object: S3Object {
                    e_tag: None,
                    key: Some(String::from("")),
                    sequencer: None,
                    size: None,
                    url_decoded_key: None,
                    version_id: None,
                },
            },
        };
        s3_event_records.push(s3_event_record);
        s3_event.records = s3_event_records;
        record.body = s3_event;
        records.push(record);
        payload.records = records;

        let event = LambdaEvent::new(payload, context);
        let _ = handler::handle(event).await;
    }
}
