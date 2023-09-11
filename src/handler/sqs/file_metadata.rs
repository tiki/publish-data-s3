/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    pub table: String,
    pub uri: String,
    pub size: i64,
    pub count: i64,
}

impl FileMetadata {
    pub fn new() -> Self {
        FileMetadata {
            table: String::new(),
            uri: String::new(),
            size: 0,
            count: 0,
        }
    }

    pub fn with_table(mut self, table: &str) -> Self {
        self.table = String::from(table);
        self
    }

    pub fn with_uri(mut self, uri: &str) -> Self {
        self.uri = String::from(uri);
        self
    }

    pub fn with_size(mut self, size: i64) -> Self {
        self.size = size;
        self
    }

    pub fn with_count(mut self, count: i64) -> Self {
        self.count = count;
        self
    }
}
