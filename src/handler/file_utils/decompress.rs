/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use aws_sdk_s3::primitives::ByteStream;
use tokio_util::io::StreamReader;
use bytes::Bytes;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use async_compression::tokio::bufread::GzipDecoder;

pub trait Decompress {
    fn gzip(&mut self) -> Lines<BufReader<GzipDecoder<&mut StreamReader<ByteStream, Bytes>>>>;
}

impl Decompress for StreamReader<ByteStream, Bytes> {
    fn gzip(&mut self) -> Lines<BufReader<GzipDecoder<&mut StreamReader<ByteStream, Bytes>>>> {
        BufReader::new(GzipDecoder::new(self)).lines()
    }
}