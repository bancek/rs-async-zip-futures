// Copyright (c) 2021-2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! An asynchronous ZIP archive reading/writing crate powered by `futures-rs`.
//!
//! Forked from [`rs-async-zip`](https://github.com/Majored/rs-async-zip). `tokio` was replaced with `futures-rs`.
//!
//! ## Features
//! - Support for Stored compression method.
//! - Support for writing streams using data descriptors.
//! - Initial support for ZIP64 writing.
//! - Aims for reasonable [specification](https://github.com/bancek/rs-async-zip-futures/blob/main/SPECIFICATION.md) compliance.
//!
//! ## Installation
//!
//! ```toml
//! [dependencies]
//! async_zip_futures = { version = "0.0.13", features = ["full"] }
//! ```
//!
//! ### Feature Flags
//! - `full` - Enables all below features.
//! - `chrono` - Enables support for parsing dates via `chrono`.
//!
//! [Read more.](https://github.com/bancek/rs-async-zip-futures)

pub mod error;
pub mod write;

pub(crate) mod entry;
pub(crate) mod read;
pub(crate) mod spec;

#[cfg(test)]
pub(crate) mod tests;

pub use crate::spec::attribute::AttributeCompatibility;
pub use crate::spec::compression::Compression;
pub use crate::spec::date::ZipDateTime;

pub use crate::entry::{builder::ZipEntryBuilder, ZipEntry};
