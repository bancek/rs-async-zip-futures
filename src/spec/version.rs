// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::entry::ZipEntry;

pub(crate) const SPEC_VERSION_MADE_BY: u16 = 63;

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#443
pub fn as_needed_to_extract(entry: &ZipEntry) -> u16 {
    let mut version = 10;

    if entry.filename().ends_with('/') {
        version = std::cmp::max(version, 20);
    }

    version
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#442
pub fn as_made_by() -> u16 {
    // Default to UNIX mapping for the moment.
    3 << 8 | SPEC_VERSION_MADE_BY
}
