// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

/// A compression method supported by this crate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    Stored,
}

impl From<&Compression> for u16 {
    // Convert a supported compression method into its relevant u16 stored with little endianness.
    // https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#445
    fn from(compression: &Compression) -> u16 {
        match compression {
            Compression::Stored => 0,
        }
    }
}

impl From<Compression> for u16 {
    fn from(compression: Compression) -> u16 {
        (&compression).into()
    }
}
