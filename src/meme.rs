use crate::jpeg::decoder::AppMarkerJpegDecoder;
use crate::jpeg::encoder::AppMarkerJpegEncoder;
use crate::jpeg::AppMarkerConfig;
use image::codecs::jpeg::JpegDecoder;
use image::{DynamicImage, ImageDecoder};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Cursor};
use std::os::unix::prelude::OpenOptionsExt;
use std::path::{Path, PathBuf};

pub static TRADE_OFFER: &[u8] = include_bytes!("../resources/trade-offer.jpg");

pub static BORROW_CHECKER: &[u8] = include_bytes!("../resources/borrow-checker.jpg");

pub static RUST_EXPERT: &[u8] = include_bytes!("../resources/rust-expert.jpg");

pub static DEBUG: &[u8] = include_bytes!("../resources/debug.jpg");

pub static RELEASE: &[u8] = include_bytes!("../resources/release.jpg");

#[derive(Debug, Clone)]
pub struct Meme {
    pub content: Vec<u8>,
}

impl Meme {
    pub fn new(s: impl AsRef<str>) -> anyhow::Result<Self> {
        let s = s.as_ref();
        let content = if let Ok(url) = reqwest::Url::parse(s) {
            log::debug!("Requesting meme from {:?}", url);
            reqwest::blocking::get(url)?.bytes()?.to_vec()
        } else {
            match s.to_lowercase().replace("-", "").as_str() {
                "debug" => DEBUG.to_vec(),
                "release" => RELEASE.to_vec(),
                "trader" => TRADE_OFFER.to_vec(),
                "expert" | "rustexpert" => RUST_EXPERT.to_vec(),
                "borrowchecker" => BORROW_CHECKER.to_vec(),
                _ => {
                    log::debug!("Reading meme file {}", s);
                    std::fs::read(s)?
                }
            }
        };
        Ok(Self { content })
    }

    /// Puts the bin file into the meme and write to dest
    pub fn write_with_bin_to(
        &self,
        bin: impl AsRef<Path>,
        dest: impl AsRef<Path>,
    ) -> anyhow::Result<PathBuf> {
        let cursor = Cursor::new(&self.content);
        let decoder = JpegDecoder::new(cursor)?;
        let color_type = decoder.color_type();
        let (width, height) = decoder.dimensions();
        let img = DynamicImage::from_decoder(decoder)?;

        log::debug!("Reading cargo binary from `{}`", bin.as_ref().display());
        let bin = BufReader::new(File::open(bin)?);

        let dest = dest.as_ref();
        log::debug!("Creating meme exe at `{}`", dest.display());
        let mut out = BufWriter::new(File::create(dest)?);

        let mut encoder =
            AppMarkerJpegEncoder::new_with_quality(&mut out, bin, AppMarkerConfig::default(), 100);

        encoder.encode(img.as_bytes(), width, height, color_type)?;

        Ok(dest.to_path_buf())
    }

    /// Decodes the binary file from a meme jpeg and writes it as executable to dest
    pub fn decode_bin_to(
        meme: impl AsRef<Path>,
        dest: impl AsRef<Path>,
    ) -> anyhow::Result<PathBuf> {
        let meme = meme.as_ref();
        let dest = dest.as_ref();
        log::debug!("Creating executable `{}`", dest.display());
        let f = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o777)
                .open(dest)?,
        );
        let output = BufWriter::new(f);
        let input = BufReader::new(File::open(meme)?);
        log::debug!("Decoding meme binary from `{}`", meme.display());
        AppMarkerJpegDecoder::new(input, output, AppMarkerConfig::default())?;
        Ok(dest.to_path_buf())
    }
}
