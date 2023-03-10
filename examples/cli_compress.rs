// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

#[tokio::main]
async fn main() {
    if let Err(err) = inner::run().await {
        eprintln!("Error: {}", err);
        eprintln!("Usage: cli_compress <input file or directory> <output ZIP file name>");
        std::process::exit(1);
    }
}

mod inner {

    use async_zip_futures::write::ZipFileWriter;
    use async_zip_futures::{Compression, ZipEntryBuilder};
    use futures::io;
    use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

    use std::path::{Path, PathBuf};

    use anyhow::{anyhow, bail, Result};
    use tokio::fs::File;

    pub async fn run() -> Result<()> {
        let mut args = std::env::args().skip(1);

        let input_str = args.next().ok_or(anyhow!("No input file or directory specified."))?;
        let input_path = Path::new(&input_str);

        let output_str = args.next().ok_or(anyhow!("No output file specified."))?;
        let output_path = Path::new(&output_str);

        let input_pathbuf = input_path.canonicalize().map_err(|_| anyhow!("Unable to canonicalise input path."))?;
        let input_path = input_pathbuf.as_path();

        if output_path.exists() {
            bail!("The output file specified already exists.");
        }
        if !input_path.exists() {
            bail!("The input file or directory specified doesn't exist.");
        }

        let file = File::create(output_path).await?;

        let mut output_writer = ZipFileWriter::new(file.compat());

        if input_path.is_dir() {
            handle_directory(input_path, &mut output_writer).await?;
        } else {
            handle_singular(input_path, &mut output_writer).await?;
        }

        output_writer.close().await?;
        println!("Successfully written ZIP file '{}'.", output_path.display());

        Ok(())
    }

    async fn handle_singular(input_path: &Path, writer: &mut ZipFileWriter<Compat<File>>) -> Result<()> {
        let filename = input_path.file_name().ok_or(anyhow!("Input path terminates in '...'."))?;
        let filename = filename.to_str().ok_or(anyhow!("Input path not valid UTF-8."))?;

        write_entry(filename, input_path, writer).await
    }

    async fn handle_directory(input_path: &Path, writer: &mut ZipFileWriter<Compat<File>>) -> Result<()> {
        let entries = walk_dir(input_path.into()).await?;
        let input_dir_str = input_path.as_os_str().to_str().ok_or(anyhow!("Input path not valid UTF-8."))?;

        for entry_path_buf in entries {
            let entry_path = entry_path_buf.as_path();
            let entry_str = entry_path.as_os_str().to_str().ok_or(anyhow!("Directory file path not valid UTF-8."))?;

            if !entry_str.starts_with(input_dir_str) {
                bail!("Directory file path does not start with base input directory path.");
            }

            let entry_str = &entry_str[input_dir_str.len() + 1..];
            write_entry(entry_str, entry_path, writer).await?;
        }

        Ok(())
    }

    async fn write_entry(filename: &str, input_path: &Path, writer: &mut ZipFileWriter<Compat<File>>) -> Result<()> {
        let input_file = File::open(input_path).await?;

        let builder = ZipEntryBuilder::new(filename.into(), Compression::Stored);

        let mut stream_writer = writer.write_entry_stream(builder).await?;

        io::copy(input_file.compat(), &mut stream_writer).await?;

        stream_writer.close().await?;

        Ok(())
    }

    async fn walk_dir(dir: PathBuf) -> Result<Vec<PathBuf>> {
        let mut dirs = vec![dir];
        let mut files = vec![];

        while !dirs.is_empty() {
            let mut dir_iter = tokio::fs::read_dir(dirs.remove(0)).await?;

            while let Some(entry) = dir_iter.next_entry().await? {
                let entry_path_buf = entry.path();

                if entry_path_buf.is_dir() {
                    dirs.push(entry_path_buf);
                } else {
                    files.push(entry_path_buf);
                }
            }
        }

        Ok(files)
    }
}
