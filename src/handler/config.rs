/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use crate::handler::file_utils::{batch::FileFormat, decompress::Compression};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub bucket: String,
    pub table: String,
    pub compression: Compression,
    pub file_type: FileFormat,
    pub region: String,
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

    pub fn from_env() -> Self {
        let bucket = env!("TIKI_BUCKET", "Please set TIKI_BUCKET");
        let table = env!("TIKI_TABLE", "Please set TIKI_TABLE");
        let file_type = env!("TIKI_FILE_TYPE", "Please set TIKI_FILE_TYPE");
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
        Config {
            bucket: String::from(bucket),
            table: String::from(table),
            compression: compression,
            region: region,
            file_type: match file_type {
                "csv" => FileFormat::CSV,
                _ => panic!("Unsupported TIKI_FILE_TYPE"),
            },
        }
    }
}
