use std::borrow::Cow;

pub mod decoder;
pub mod encoder;
mod transform;

/// There are several APP markers used by "popular" tools https://github.com/corkami/formats/blob/master/image/jpeg.md#jpeg-1997
///
/// randomly chose 8 (what is a SPIFF?) to store all data in consecutive APP8 markers
pub const APP7: u8 = 0xE8;

/// app0 marker
pub const APP0_DECIMAL: u8 = 224;

/// It's common practice to put the vendor name at the start
static MEMEX_VENDOR: &[u8] = b"memex\0";

pub struct AppMarkerConfig {
    pub vendor: Cow<'static, [u8]>,
    pub marker: u8,
}

impl AppMarkerConfig {
    /// Maximum amount of data that can we can store inside a single APP marker.
    /// The length of the segment that follows the app marker is encoded as be u16 (2 bytes).
    /// The length is part of the segment which leaves us with 2^16-2-len(vendor) as usable space.
    pub fn data_len(&self) -> usize {
        self.segment_data_len() - self.vendor.len()
    }

    /// The length of the segment including the vendor name
    pub fn segment_data_len(&self) -> usize {
        u16::MAX as usize - 2
    }
}

impl Default for AppMarkerConfig {
    fn default() -> Self {
        Self {
            vendor: MEMEX_VENDOR.into(),
            marker: APP7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        jpeg::decoder::AppMarkerJpegDecoder, jpeg::encoder::AppMarkerJpegEncoder, meme::TRADE_OFFER,
    };
    use image::{
        codecs::jpeg::{JpegDecoder, JpegEncoder},
        ColorType, DynamicImage, ImageDecoder,
    };
    use std::{
        fs::{File, OpenOptions},
        io::{BufReader, BufWriter, Cursor, Read},
        os::unix::prelude::OpenOptionsExt,
    };

    #[test]
    #[ignore]
    fn decode_encode() {
        let cursor = Cursor::new(TRADE_OFFER);
        let decoder = JpegDecoder::new(cursor).unwrap();
        let color_type = decoder.color_type();

        let img = DynamicImage::from_decoder(decoder).unwrap();

        let meta = BufReader::new(File::open("./resources/demo").unwrap());
        let output = File::create("./resources/encoded.jpeg").unwrap();
        let mut out = BufWriter::new(output);
        let mut encoder =
            AppMarkerJpegEncoder::new_with_quality(&mut out, meta, AppMarkerConfig::default(), 100);
        encoder
            .encode(img.as_bytes(), 500, 654, color_type)
            .unwrap();
    }

    #[test]
    #[ignore]
    fn demo_jpeg() {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o777)
            .open("./resources/decoded")
            .unwrap();
        let output = BufWriter::new(f);
        let input = BufReader::new(File::open("./resources/encoded.jpeg").unwrap());
        let decoder = AppMarkerJpegDecoder::new(input, output, AppMarkerConfig::default()).unwrap();
        decoder.dimensions();
        // let img = DynamicImage::from_decoder(decoder).unwrap();
    }
}
