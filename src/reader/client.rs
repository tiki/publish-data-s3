/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use aws_config::from_env;
use aws_sdk_s3;
use bytes::Bytes;
use tokio_util::io::StreamReader;

pub struct Client {
    s3: aws_sdk_s3::Client,
}

impl Client {
    pub async fn new(region: aws_sdk_s3::config::Region) -> Self {
        let config = from_env().region(region).load().await;
        Client { s3: aws_sdk_s3::Client::new(&config) }
    }

    pub async fn read(&self, bucket: &str, key: &str)
        -> Result<StreamReader<aws_sdk_s3::primitives::ByteStream, Bytes>,
            aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::get_object::GetObjectError>>
    {
        let res = self.s3
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;
        Ok(StreamReader::new(res.body))
    }
}
