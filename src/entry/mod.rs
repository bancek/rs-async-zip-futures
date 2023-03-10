// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

pub mod builder;

use crate::entry::builder::ZipEntryBuilder;
use crate::spec::{attribute::AttributeCompatibility, date::ZipDateTime, header::ExtraField, Compression};

/// An immutable store of data about a ZIP entry.
///
/// This type cannot be directly constructed so instead, the [`ZipEntryBuilder`] must be used. Internally this builder
/// stores a [`ZipEntry`] so conversions between these two types via the [`From`] implementations will be
/// non-allocating.
#[derive(Clone, Debug)]
pub struct ZipEntry {
    pub(crate) filename: String,
    pub(crate) compression: Compression,
    pub(crate) crc32: u32,
    pub(crate) uncompressed_size: u64,
    pub(crate) compressed_size: u64,
    pub(crate) attribute_compatibility: AttributeCompatibility,
    pub(crate) last_modification_date: ZipDateTime,
    pub(crate) internal_file_attribute: u16,
    pub(crate) external_file_attribute: u32,
    pub(crate) extra_fields: Vec<ExtraField>,
    pub(crate) comment: String,
}

impl From<ZipEntryBuilder> for ZipEntry {
    fn from(builder: ZipEntryBuilder) -> Self {
        builder.0
    }
}

impl ZipEntry {
    pub(crate) fn new(filename: String, compression: Compression) -> Self {
        ZipEntry {
            filename,
            compression,
            crc32: 0,
            uncompressed_size: 0,
            compressed_size: 0,
            attribute_compatibility: AttributeCompatibility::Unix,
            last_modification_date: ZipDateTime::default(),
            internal_file_attribute: 0,
            external_file_attribute: 0,
            extra_fields: Vec::new(),
            comment: String::new(),
        }
    }

    /// Returns the entry's filename.
    ///
    /// ## Note
    /// This will return the raw filename stored during ZIP creation. If calling this method on entries retrieved from
    /// untrusted ZIP files, the filename should be sanitised before being used as a path to prevent [directory
    /// traversal attacks](https://en.wikipedia.org/wiki/Directory_traversal_attack).
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Returns the entry's compression method.
    pub fn compression(&self) -> Compression {
        self.compression
    }

    /// Returns the entry's CRC32 value.
    pub fn crc32(&self) -> u32 {
        self.crc32
    }

    /// Returns the entry's uncompressed size.
    pub fn uncompressed_size(&self) -> u64 {
        self.uncompressed_size
    }

    /// Returns the entry's compressed size.
    pub fn compressed_size(&self) -> u64 {
        self.compressed_size
    }

    /// Returns the entry's attribute's host compatibility.
    pub fn attribute_compatibility(&self) -> AttributeCompatibility {
        self.attribute_compatibility
    }

    /// Returns the entry's last modification time & date.
    pub fn last_modification_date(&self) -> &ZipDateTime {
        &self.last_modification_date
    }

    /// Returns the entry's internal file attribute.
    pub fn internal_file_attribute(&self) -> u16 {
        self.internal_file_attribute
    }

    /// Returns the entry's external file attribute
    pub fn external_file_attribute(&self) -> u32 {
        self.external_file_attribute
    }

    /// Returns the entry's extra field data.
    pub fn extra_fields(&self) -> &[ExtraField] {
        &self.extra_fields
    }

    /// Returns the entry's file comment.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Returns the entry's integer-based UNIX permissions.
    ///
    /// # Note
    /// This will return None if the attribute host compatibility is not listed as Unix.
    pub fn unix_permissions(&self) -> Option<u16> {
        if !matches!(self.attribute_compatibility, AttributeCompatibility::Unix) {
            return None;
        }

        Some(((self.external_file_attribute) >> 16) as u16)
    }

    /// Returns whether or not the entry represents a directory.
    pub fn dir(&self) -> bool {
        self.filename.ends_with('/')
    }
}
