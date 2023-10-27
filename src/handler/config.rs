/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::handler::file_utils::{batch::FileFormat, decompress::Compression};
use serde_json::from_str;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub bucket: String,
    pub table: String,
    pub compression: Compression,
    pub file_type: FileFormat,
    pub region: String,
    pub queue: String,
    pub max_rows: usize,
}

#[allow(unused)]
impl Config {
    pub fn default() -> Self {
        Config {
            bucket: String::new(),
            table: String::new(),
            compression: Compression::None,
            file_type: FileFormat::CSV,
            region: String::from("us-east-2"),
            queue: String::new(),
            max_rows: 500000,
        }
    }

    pub fn with_bucket(mut self, bucket: &str) -> Self {
        self.bucket = String::from(bucket);
        self
    }

    pub fn with_table(mut self, table: &str) -> Self {
        self.table = String::from(table);
        self
    }

    pub fn with_compression(mut self, compression: Compression) -> Self {
        self.compression = compression;
        self
    }

    pub fn with_file_type(mut self, file_type: FileFormat) -> Self {
        self.file_type = file_type;
        self
    }

    pub fn with_region(mut self, region: &str) -> Self {
        self.region = String::from(region);
        self
    }

    pub fn with_queue(mut self, queue: &str) -> Self {
        self.queue = String::from(queue);
        self
    }

    pub fn with_max_rows(mut self, max_rows: usize) -> Self {
        self.max_rows = max_rows;
        self
    }

    pub fn from_env() -> Self {
        let bucket = match env::var("TIKI_BUCKET") {
            Ok(table) => table,
            Err(_) => panic!("Please set TIKI_BUCKET"),
        };
        let table = match env::var("TIKI_TABLE") {
            Ok(table) => table,
            Err(_) => panic!("Please set TIKI_TABLE"),
        };
        let queue = match env::var("TIKI_QUEUE") {
            Ok(queue) => queue,
            Err(_) => panic!("Please set TIKI_QUEUE"),
        };
        let file_type = match env::var("TIKI_FILE_TYPE") {
            Ok(file_type) => match file_type.as_str() {
                "csv" => FileFormat::CSV,
                _ => panic!("Unsupported TIKI_FILE_TYPE"),
            },
            Err(_) => panic!("Please set TIKI_FILE_TYPE"),
        };
        let compression = match env::var("TIKI_COMPRESSION") {
            Ok(compression) => match compression.as_str() {
                "gzip" => Compression::GZip,
                "none" => Compression::None,
                _ => Compression::None,
            },
            Err(_) => Compression::None,
        };
        let region = match env::var("TIKI_REGION") {
            Ok(region) => region,
            Err(_) => String::from("us-east-2"),
        };
        let max_rows = match env::var("TIKI_MAX_ROWS") {
            Ok(max_rows) => {
                let res = from_str::<usize>(&max_rows);
                match res {
                    Ok(max_rows) => max_rows,
                    Err(err) => 100000,
                }
            }
            Err(_) => 100000,
        };
        Config {
            bucket: String::from(bucket),
            table: String::from(table),
            compression: compression,
            region: region,
            file_type: file_type,
            queue: queue,
            max_rows: max_rows,
        }
    }
}
