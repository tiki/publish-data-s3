/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use aws_config::from_env;
use aws_sdk_s3;
use aws_sdk_s3::types::ObjectAttributes;
use bytes::Bytes;
use std::error::Error;
use tokio_util::io::StreamReader;

pub struct Client {
    s3: aws_sdk_s3::Client,
}

impl Client {
    pub async fn new(region: &str) -> Self {
        let config = from_env()
            .region(aws_sdk_s3::config::Region::new(String::from(region)))
            .load()
            .await;
        Client {
            s3: aws_sdk_s3::Client::new(&config),
        }
    }

    pub async fn read(
        &self,
        bucket: String,
        key: String,
    ) -> Result<StreamReader<aws_sdk_s3::primitives::ByteStream, Bytes>, Box<dyn Error>> {
        let res = self.s3.get_object().bucket(bucket).key(key).send().await?;
        Ok(StreamReader::new(res.body))
    }

    pub async fn read_metadata(
        &self,
        bucket: String,
        key: String,
    ) -> Result<
        aws_sdk_s3::operation::get_object_attributes::GetObjectAttributesOutput,
        Box<dyn Error>,
    > {
        let res = self
            .s3
            .get_object_attributes()
            .bucket(bucket)
            .key(key)
            .object_attributes(ObjectAttributes::ObjectSize)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn write(
        &self,
        bucket: String,
        key: String,
        body: Vec<u8>,
    ) -> Result<aws_sdk_s3::operation::put_object::PutObjectOutput, Box<dyn Error>> {
        let res = self
            .s3
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(aws_sdk_s3::primitives::ByteStream::from(body))
            .send()
            .await?;
        Ok(res)
    }
}
