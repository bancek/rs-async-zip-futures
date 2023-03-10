// Copyright Cognite AS, 2023

use crate::spec::header::{ExtraField, HeaderId, UnknownExtraField, Zip64ExtendedInformationExtraField};

impl From<u16> for HeaderId {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => Self::Zip64ExtendedInformationExtraField,
            other => Self::Other(other),
        }
    }
}

impl From<HeaderId> for u16 {
    fn from(value: HeaderId) -> Self {
        match value {
            HeaderId::Zip64ExtendedInformationExtraField => 0x0001,
            HeaderId::Other(other) => other,
        }
    }
}

pub(crate) trait ExtraFieldAsBytes {
    fn as_bytes(&self) -> Vec<u8>;

    fn count_bytes(&self) -> usize;
}

impl ExtraFieldAsBytes for &[ExtraField] {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        for field in self.iter() {
            buffer.append(&mut field.as_bytes());
        }
        buffer
    }

    fn count_bytes(&self) -> usize {
        self.iter().map(|field| field.count_bytes()).sum()
    }
}

impl ExtraFieldAsBytes for ExtraField {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            ExtraField::Zip64ExtendedInformationExtraField(field) => field.as_bytes(),
            ExtraField::UnknownExtraField(field) => field.as_bytes(),
        }
    }

    fn count_bytes(&self) -> usize {
        match self {
            ExtraField::Zip64ExtendedInformationExtraField(field) => field.count_bytes(),
            ExtraField::UnknownExtraField(field) => field.count_bytes(),
        }
    }
}

impl ExtraFieldAsBytes for UnknownExtraField {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header_id: u16 = self.header_id.into();
        bytes.append(&mut header_id.to_le_bytes().to_vec());
        bytes.append(&mut self.data_size.to_le_bytes().to_vec());
        bytes.append(&mut self.content.clone());

        bytes
    }

    fn count_bytes(&self) -> usize {
        4 + self.content.len()
    }
}

impl ExtraFieldAsBytes for Zip64ExtendedInformationExtraField {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header_id: u16 = self.header_id.into();
        bytes.append(&mut header_id.to_le_bytes().to_vec());
        bytes.append(&mut self.data_size.to_le_bytes().to_vec());
        bytes.append(&mut self.uncompressed_size.to_le_bytes().to_vec());
        bytes.append(&mut self.compressed_size.to_le_bytes().to_vec());
        if let Some(relative_header_offset) = &self.relative_header_offset {
            bytes.append(&mut relative_header_offset.to_le_bytes().to_vec());
        }
        if let Some(disk_start_number) = &self.disk_start_number {
            bytes.append(&mut disk_start_number.to_le_bytes().to_vec());
        }

        bytes
    }

    fn count_bytes(&self) -> usize {
        20 + self.relative_header_offset.map(|_| 8).unwrap_or_default()
            + self.disk_start_number.map(|_| 8).unwrap_or_default()
    }
}
