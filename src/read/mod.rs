// Copyright (c) 2022-2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::spec::header::{ExtraField, Zip64ExtendedInformationExtraField};

pub(crate) fn get_zip64_extra_field_mut(
    extra_fields: &mut [ExtraField],
) -> Option<&mut Zip64ExtendedInformationExtraField> {
    for field in extra_fields {
        if let ExtraField::Zip64ExtendedInformationExtraField(zip64field) = field {
            return Some(zip64field);
        }
    }
    None
}
