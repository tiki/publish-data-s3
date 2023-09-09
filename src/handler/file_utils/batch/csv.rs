/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use futures::future::BoxFuture;
use serde::de::DeserializeOwned;
use std::error::Error;
use tokio::io::{AsyncBufRead, Lines};

pub async fn process<
    'a,
    R: AsyncBufRead + Unpin,
    T: DeserializeOwned,
    F: Fn(Vec<T>) -> BoxFuture<'a, ()>,
>(
    lines: &mut Lines<R>,
    size: usize,
    processor: F,
) {
    let mut chunk: Vec<String> = Vec::new();
    let header = lines.next_line().await.unwrap_or_else(|_| None);
    if let Some(header) = header {
        let header = header;
        chunk.push(header.clone());
        while let Some(line) = lines.next_line().await.unwrap_or_else(|_| None) {
            if chunk.len() <= size {
                chunk.push(line);
            } else {
                let records = deserialize::<T>(chunk);
                if let Ok(records) = records {
                    processor(records).await;
                }
                chunk = Vec::new();
                chunk.push(header.clone())
            }
        }
    }
    if chunk.len() > 0 {
        let records = deserialize::<T>(chunk);
        if let Ok(records) = records {
            processor(records);
        }
    }
}

fn deserialize<T: DeserializeOwned>(rows: Vec<String>) -> Result<Vec<T>, Box<dyn Error>> {
    let raw = rows.join("\n");
    let mut reader = csv::Reader::from_reader(raw.as_bytes());
    let mut records: Vec<T> = Vec::new();
    for result in reader.deserialize::<T>() {
        let record = result?;
        records.push(record);
    }
    Ok(records)
}
