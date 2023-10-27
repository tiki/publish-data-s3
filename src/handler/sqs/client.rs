/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::handler::sqs::file_metadata::FileMetadata;
use aws_config::from_env;
use std::error::Error;

pub struct Client {
    sqs: aws_sdk_sqs::Client,
}

impl Client {
    pub async fn new(region: &str) -> Self {
        let config = from_env()
            .region(aws_sdk_sqs::config::Region::new(String::from(region)))
            .load()
            .await;
        Client {
            sqs: aws_sdk_sqs::Client::new(&config),
        }
    }

    pub async fn write(
        &self,
        queue_url: &str,
        file_metadata: &FileMetadata,
    ) -> Result<aws_sdk_sqs::operation::send_message::SendMessageOutput, Box<dyn Error>> {
        let body = serde_json::to_string(file_metadata)?;
        let rsp = self
            .sqs
            .send_message()
            .queue_url(queue_url)
            .message_body(body)
            .message_group_id(file_metadata.table.to_string())
            .send()
            .await?;
        Ok(rsp)
    }
}
