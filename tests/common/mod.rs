// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use std::io::Cursor;
use std::io::Read;

use async_zip_futures::write::ZipFileWriter;
use async_zip_futures::Compression;
use async_zip_futures::ZipEntryBuilder;
use futures::AsyncWriteExt;

const FOLDER_PREFIX: &str = "tests/test_inputs";

const FILE_LIST: &[&str] = &[
    "sample_data/alpha/back_to_front.txt",
    "sample_data/alpha/front_to_back.txt",
    "sample_data/numeric/forward.txt",
    "sample_data/numeric/reverse.txt",
];

pub async fn compress_to_mem(compress: Compression) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(10_000);
    let mut writer = ZipFileWriter::new(&mut bytes);

    for fname in FILE_LIST {
        let content = tokio::fs::read(format!("{FOLDER_PREFIX}/{fname}")).await.unwrap();
        let opts = ZipEntryBuilder::new(fname.to_string(), compress);

        let mut entry_writer = writer.write_entry_stream(opts).await.unwrap();
        entry_writer.write_all(&content).await.unwrap();
        entry_writer.close().await.unwrap();
    }
    writer.close().await.unwrap();
    bytes
}

pub async fn check_decompress_mem(zip_data: Vec<u8>) {
    let mut zip = zip::ZipArchive::new(Cursor::new(zip_data)).unwrap();
    for idx in 0..zip.len() {
        let mut file = zip.by_index(idx).unwrap();
        if file.is_dir() {
            continue;
        }
        let fname = file.name().to_owned();
        let mut output = String::new();
        let _ = file.read_to_string(&mut output).unwrap();
        let fs_file = format!("{FOLDER_PREFIX}/{fname}");
        let expected = tokio::fs::read_to_string(fs_file).await.unwrap();
        assert_eq!(output, expected, "for {fname}, expect zip data to match file data");
    }
}
