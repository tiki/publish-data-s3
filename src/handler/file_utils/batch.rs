/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod csv;

use async_trait::async_trait;
use futures::future::BoxFuture;
use serde::de::DeserializeOwned;
use tokio::io::{AsyncBufRead, Lines};

#[async_trait(?Send)]
pub trait Batch<R: AsyncBufRead + Unpin> {
    async fn process_csv<'a, T: DeserializeOwned, F: Fn(Vec<T>) -> BoxFuture<'a, ()>>(
        &mut self,
        size: usize,
        processor: F,
    );

    async fn process<'a, T: DeserializeOwned, F: Fn(Vec<T>) -> BoxFuture<'a, ()>>(
        &mut self,
        size: usize,
        format: &str,
        processor: F,
    );
}

#[async_trait(?Send)]
impl<R: AsyncBufRead + Unpin> Batch<R> for Lines<R> {
    async fn process_csv<'a, T: DeserializeOwned, F: Fn(Vec<T>) -> BoxFuture<'a, ()>>(
        &mut self,
        size: usize,
        processor: F,
    ) {
        csv::process::<R, T, F>(self, size, processor).await
    }

    async fn process<'a, T: DeserializeOwned, F: Fn(Vec<T>) -> BoxFuture<'a, ()>>(
        &mut self,
        size: usize,
        format: &str,
        processor: F,
    ) {
        match format {
            "csv" => csv::process::<R, T, F>(self, size, processor).await,
            _ => {
                panic!("Unsupported batch processing format: {}", format);
            }
        }
    }
}
