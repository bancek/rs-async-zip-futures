// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use async_zip_futures::Compression;

mod common;

#[tokio::test]
async fn zip_store_in_out() {
    let zip_data = common::compress_to_mem(Compression::Stored).await;
    common::check_decompress_mem(zip_data).await
}
