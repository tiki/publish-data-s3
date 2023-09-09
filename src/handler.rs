/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use lambda_runtime::{Error, LambdaEvent};
use aws_lambda_events::{event::sqs::SqsEventObj, s3::S3Event};
use crate::reader::{client::Client, decompress::Decompress, batch::Batch};
use tiki_private_ingest::schema::Schema;

pub async fn handle(event: LambdaEvent<SqsEventObj<S3Event>>) -> Result<(), Error> {
    //for record in &event.payload.records {}

    let client = Client::new("us-east-2").await;
    let mut stream = client.read(
        "mytiki-staging-attain",
        "weekly_delivery/2023/8/29/tiki_user_demo_2023-08-21.csv.gz")
        .await
        .unwrap();

    stream.gzip().process_csv(100, |users:Vec<Schema>| {
        println!("{} PROCESSED", users.len());
    }).await;

    Ok(())
}
