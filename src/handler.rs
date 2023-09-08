/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use lambda_runtime::{Error, LambdaEvent};
use aws_lambda_events::{event::sqs::SqsEventObj, s3::S3Event};

pub async fn handle(event: LambdaEvent<SqsEventObj<S3Event>>) -> Result<(), Error> {
    for record in &event.payload.records {}
    Ok(())
}
