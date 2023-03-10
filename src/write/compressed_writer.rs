// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use futures::AsyncWrite;

use crate::spec::Compression;
use crate::write::io::offset::AsyncOffsetWriter;

use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

pub enum CompressedAsyncWriter<'b, W: AsyncWrite + Unpin> {
    Stored(ShutdownIgnoredWriter<&'b mut AsyncOffsetWriter<W>>),
}

impl<'b, W: AsyncWrite + Unpin> CompressedAsyncWriter<'b, W> {
    pub fn from_raw(writer: &'b mut AsyncOffsetWriter<W>, compression: Compression) -> Self {
        match compression {
            Compression::Stored => CompressedAsyncWriter::Stored(ShutdownIgnoredWriter(writer)),
        }
    }

    pub fn into_inner(self) -> &'b mut AsyncOffsetWriter<W> {
        match self {
            CompressedAsyncWriter::Stored(inner) => inner.into_inner(),
        }
    }
}

impl<'b, W: AsyncWrite + Unpin> AsyncWrite for CompressedAsyncWriter<'b, W> {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<std::result::Result<usize, Error>> {
        match *self {
            CompressedAsyncWriter::Stored(ref mut inner) => Pin::new(inner).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<std::result::Result<(), Error>> {
        match *self {
            CompressedAsyncWriter::Stored(ref mut inner) => Pin::new(inner).poll_flush(cx),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<std::result::Result<(), Error>> {
        match *self {
            CompressedAsyncWriter::Stored(ref mut inner) => Pin::new(inner).poll_close(cx),
        }
    }
}

pub struct ShutdownIgnoredWriter<W: AsyncWrite + Unpin>(W);

impl<W: AsyncWrite + Unpin> ShutdownIgnoredWriter<W> {
    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for ShutdownIgnoredWriter<W> {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<std::result::Result<usize, Error>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<std::result::Result<(), Error>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<std::result::Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}
