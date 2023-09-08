/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;
use tokio::io::{AsyncBufRead, Lines};
use serde::de::DeserializeOwned;

pub async fn process<R: AsyncBufRead + Unpin, T: DeserializeOwned>(
    lines: &mut Lines<R>,
    size: usize,
    processor: fn(Vec<T>)
) {
    let mut chunk: Vec<String> = Vec::new();
    let header = lines.next_line().await.unwrap_or_else(|_| { None });
    if let Some(header) = header {
        let header = header;
        chunk.push(header.clone());
        while let Some(line) = lines.next_line().await.unwrap_or_else(|_| { None }) {
            if chunk.len() <= size {
                chunk.push(line);
            } else {
                let records = deserialize::<T>(chunk);
                if let Ok(records) = records { processor(records); }
                chunk = Vec::new();
                chunk.push(header.clone())
            }
        }
    }
    if chunk.len() > 0 {
        let records = deserialize::<T>(chunk);
        if let Ok(records) = records { processor(records); }
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


