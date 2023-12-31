/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use async_compression::tokio::bufread::GzipDecoder;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader, Lines};
use tokio_util::io::StreamReader;

#[derive(Debug, Copy, Clone)]
pub enum Compression {
    GZip,
    None,
}

pub trait Decompress {
    fn gzip(&mut self) -> Lines<BufReader<GzipDecoder<&mut StreamReader<ByteStream, Bytes>>>>;
    fn from<'a>(&'a mut self, format: Compression) -> Box<dyn AsyncBufRead + Unpin + 'a>;
}

impl Decompress for StreamReader<ByteStream, Bytes> {
    fn gzip(&mut self) -> Lines<BufReader<GzipDecoder<&mut StreamReader<ByteStream, Bytes>>>> {
        BufReader::new(GzipDecoder::new(self)).lines()
    }

    fn from<'a>(&'a mut self, format: Compression) -> Box<dyn AsyncBufRead + Unpin + 'a> {
        match format {
            Compression::GZip => Box::new(BufReader::new(GzipDecoder::new(self))),
            Compression::None => Box::new(BufReader::new(self)),
        }
    }
}
