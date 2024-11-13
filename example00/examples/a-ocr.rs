use std::path::PathBuf;

use anyhow::Result;

use windows::Media::Ocr::OcrEngine;
//use windows::core::*;
use windows::core::HSTRING;
use windows::Graphics::Imaging::BitmapDecoder;
use windows::Storage::{FileAccessMode, StorageFile};

#[tokio::main]
async fn main() -> Result<()> {
    let mut message: PathBuf = std::env::current_dir().unwrap();
    message.push("message.png");

    let file =
        StorageFile::GetFileFromPathAsync(&HSTRING::from(message.to_str().unwrap()))?.await?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.await?;

    let decode = BitmapDecoder::CreateAsync(&stream)?.await?;
    let bitmap = decode.GetSoftwareBitmapAsync()?.await?;

    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    let result = engine.RecognizeAsync(&bitmap)?.await?;

    println!("{}", result.Text()?);

    Ok(())
}
