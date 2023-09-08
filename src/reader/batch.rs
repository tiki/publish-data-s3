/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use tokio::io::{AsyncBufRead, Lines};
use crate::reader::file_type::csv;

#[async_trait(?Send)]
pub trait Batch<R: AsyncBufRead + Unpin> {
    async fn process_csv<T: DeserializeOwned>(
        &mut self,
        size: usize,
        processor: fn(Vec<T>));
}

#[async_trait(?Send)]
impl <R: AsyncBufRead + Unpin> Batch<R> for Lines<R> {
    async fn process_csv<T: DeserializeOwned>(&mut self, size: usize, processor: fn(Vec<T>)) {
        csv::process::<R,T>(self, size, processor).await
    }
}
