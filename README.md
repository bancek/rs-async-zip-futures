# async_zip_futures
[![Crates.io](https://img.shields.io/crates/v/async_zip_futures?style=flat-square)](https://crates.io/crates/async_zip_futures)
[![Crates.io](https://img.shields.io/crates/d/async_zip_futures?style=flat-square)](https://crates.io/crates/async_zip_futures)
[![docs.rs](https://img.shields.io/docsrs/async_zip_futures?style=flat-square)](https://docs.rs/async_zip_futures/)
[![GitHub Workflow Status (branch)](https://img.shields.io/github/actions/workflow/status/bancek/rs-async-zip-futures/ci-linux.yml?branch=main&style=flat-square)](https://github.com/bancek/rs-async-zip-futures/actions?query=branch%3Amain)
[![GitHub](https://img.shields.io/github/license/bancek/rs-async-zip-futures?style=flat-square)](https://github.com/bancek/rs-async-zip-futures/blob/main/LICENSE)

An asynchronous ZIP archive reading/writing crate powered by [`futures-rs`](https://crates.io/crates/futures).

Forked from [`rs-async-zip`](https://github.com/Majored/rs-async-zip). `tokio` was replaced with `futures-rs`.

## Features
- Support for Stored compression method.
- Support for writing streams using data descriptors.
- Initial support for ZIP64 writing.
- Aims for reasonable [specification](https://github.com/bancek/rs-async-zip-futures/blob/main/SPECIFICATION.md) compliance.

## Installation & Basic Usage

```toml
[dependencies]
async_zip_futures = { version = "0.0.12", features = ["full"] }
```

A (soon to be) extensive list of [examples](https://github.com/bancek/rs-async-zip-futures/tree/main/examples) can be found under the `/examples` directory.

### Feature Flags
- `full` - Enables all below features.
- `chrono` - Enables support for parsing dates via `chrono`.

### Writing
```rust
use async_zip_futures::{Compression, ZipEntryBuilder, write::ZipFileWriter, error::ZipError};
use futures::AsyncWriteExt;
use tokio::fs::File;
use tokio_util::compat::TokioAsyncReadCompatExt;
...

let mut file = File::create("foo.zip").await.unwrap().compat();
let mut writer = ZipFileWriter::new(&mut file);

let data = b"This is an example file.";
let builder = ZipEntryBuilder::new(String::from("bar.txt"), Compression::Stored);

let mut entry_writer = writer.write_entry_stream(builder).await.unwrap();
entry_writer.write_all(data).await.unwrap();
entry_writer.close().await.unwrap();
writer.close().await.unwrap();
```
