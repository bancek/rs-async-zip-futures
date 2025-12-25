// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::entry::ZipEntry;
use crate::error::{Result, Zip64ErrorCase, ZipError};
use crate::spec::{
    extra_field::ExtraFieldAsBytes,
    header::{
        CentralDirectoryRecord, ExtraField, GeneralPurposeFlag, HeaderId, LocalFileHeader,
        Zip64ExtendedInformationExtraField,
    },
};
use crate::write::{CentralDirectoryEntry, ZipFileWriter};

use crate::spec::consts::{NON_ZIP64_MAX_NUM_FILES, NON_ZIP64_MAX_SIZE};
use crc32fast::Hasher;
use futures::{AsyncWrite, AsyncWriteExt};

pub struct EntryWholeWriter<'b, 'c, W: AsyncWrite + Unpin> {
    writer: &'b mut ZipFileWriter<W>,
    entry: ZipEntry,
    data: &'c [u8],
}

impl<'b, 'c, W: AsyncWrite + Unpin> EntryWholeWriter<'b, 'c, W> {
    pub fn from_raw(writer: &'b mut ZipFileWriter<W>, entry: ZipEntry, data: &'c [u8]) -> Self {
        Self { writer, entry, data }
    }

    pub async fn write(mut self) -> Result<()> {
        let (lfh_compressed_size, lfh_uncompressed_size) = if self.data.len() as u64 > NON_ZIP64_MAX_SIZE as u64
            || self.data.len() as u64 > NON_ZIP64_MAX_SIZE as u64
        {
            if self.writer.force_no_zip64 {
                return Err(ZipError::Zip64Needed(Zip64ErrorCase::LargeFile));
            }
            if !self.writer.is_zip64 {
                self.writer.is_zip64 = true;
            }
            self.entry.extra_fields.push(ExtraField::Zip64ExtendedInformationExtraField(
                Zip64ExtendedInformationExtraField {
                    header_id: HeaderId::Zip64ExtendedInformationExtraField,
                    data_size: 16,
                    uncompressed_size: self.data.len() as u64,
                    compressed_size: self.data.len() as u64,
                    relative_header_offset: None,
                    disk_start_number: None,
                },
            ));
            (NON_ZIP64_MAX_SIZE, NON_ZIP64_MAX_SIZE)
        } else {
            (self.data.len() as u32, self.data.len() as u32)
        };

        let lf_header = LocalFileHeader {
            compressed_size: lfh_compressed_size,
            uncompressed_size: lfh_uncompressed_size,
            compression: self.entry.compression().into(),
            crc: compute_crc(self.data),
            extra_field_length: self
                .entry
                .extra_fields()
                .count_bytes()
                .try_into()
                .map_err(|_| ZipError::ExtraFieldTooLarge)?,
            file_name_length: self.entry.filename().len().try_into().map_err(|_| ZipError::FileNameTooLarge)?,
            mod_time: self.entry.last_modification_date().time,
            mod_date: self.entry.last_modification_date().date,
            version: crate::spec::version::as_needed_to_extract(&self.entry),
            flags: GeneralPurposeFlag {
                data_descriptor: false,
                encrypted: false,
                filename_unicode: !self.entry.filename().is_ascii(),
            },
        };

        let header = CentralDirectoryRecord {
            v_made_by: crate::spec::version::as_made_by(),
            v_needed: lf_header.version,
            compressed_size: lf_header.compressed_size,
            uncompressed_size: lf_header.uncompressed_size,
            compression: lf_header.compression,
            crc: lf_header.crc,
            extra_field_length: lf_header.extra_field_length,
            file_name_length: lf_header.file_name_length,
            file_comment_length: self.entry.comment().len().try_into().map_err(|_| ZipError::CommentTooLarge)?,
            mod_time: lf_header.mod_time,
            mod_date: lf_header.mod_date,
            flags: lf_header.flags,
            disk_start: 0,
            inter_attr: self.entry.internal_file_attribute(),
            exter_attr: self.entry.external_file_attribute(),
            lh_offset: self.writer.writer.offset() as u32,
        };

        self.writer.writer.write_all(&crate::spec::consts::LFH_SIGNATURE.to_le_bytes()).await?;
        self.writer.writer.write_all(&lf_header.as_slice()).await?;
        self.writer.writer.write_all(self.entry.filename().as_bytes()).await?;
        self.writer.writer.write_all(&self.entry.extra_fields().as_bytes()).await?;
        self.writer.writer.write_all(self.data).await?;

        self.writer.cd_entries.push(CentralDirectoryEntry { header, entry: self.entry });
        // Ensure that we can fit this many files in this archive if forcing no zip64
        if self.writer.cd_entries.len() > NON_ZIP64_MAX_NUM_FILES as usize {
            if self.writer.force_no_zip64 {
                return Err(ZipError::Zip64Needed(Zip64ErrorCase::TooManyFiles));
            }
            if !self.writer.is_zip64 {
                self.writer.is_zip64 = true;
            }
        }

        Ok(())
    }
}

fn compute_crc(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}
