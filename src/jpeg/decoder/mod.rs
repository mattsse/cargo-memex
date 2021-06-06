// The MIT License (MIT)
//
// Copyright (c) 2014 PistonDevelopers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Modified Jpeg decoder
//!
//! Adapted from https://github.com/image-rs/image/blob/master/src/codecs/jpeg/decoder.rs

use std::convert::TryFrom;
use std::io::{self, Cursor, Read, Write};
use std::marker::PhantomData;
use std::mem;

use image::error::{
    DecodingError, ImageError, ImageResult, UnsupportedError, UnsupportedErrorKind,
};
use image::ColorType;
use image::{ImageDecoder, ImageFormat};

use crate::jpeg::AppMarkerConfig;
pub use decoder::{Decoder, ImageInfo, PixelFormat};
pub use error::{Error, UnsupportedFeature};

mod decoder;
mod error;
mod huffman;
mod idct;
mod marker;
mod parser;
mod upsampler;
mod worker;

fn read_u8<R: std::io::Read>(reader: &mut R) -> std::io::Result<u8> {
    let mut buf = [0];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_u16_from_be<R: std::io::Read>(reader: &mut R) -> std::io::Result<u16> {
    let mut buf = [0, 0];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

/// JPEG decoder
pub struct AppMarkerJpegDecoder<R, W> {
    decoder: self::Decoder<R, W>,
    metadata: self::ImageInfo,
}

impl<R: Read, W: Write> AppMarkerJpegDecoder<R, W> {
    /// Create a new decoder that decodes from the stream ```r```
    pub fn new(r: R, w: W, config: AppMarkerConfig) -> ImageResult<AppMarkerJpegDecoder<R, W>> {
        let mut decoder = self::Decoder::new(r, w, config);

        decoder.read_info().map_err(jpeg_error_to_image_error)?;
        let mut metadata = decoder.info().ok_or_else(|| {
            ImageError::Decoding(DecodingError::from_format_hint(ImageFormat::Jpeg.into()))
        })?;

        // We convert CMYK data to RGB before returning it to the user.
        if metadata.pixel_format == self::PixelFormat::CMYK32 {
            metadata.pixel_format = self::PixelFormat::RGB24;
        }

        Ok(AppMarkerJpegDecoder { decoder, metadata })
    }

    /// Configure the decoder to scale the image during decoding.
    ///
    /// This efficiently scales the image by the smallest supported
    /// scale factor that produces an image larger than or equal to
    /// the requested size in at least one axis. The currently
    /// implemented scale factors are 1/8, 1/4, 1/2 and 1.
    ///
    /// To generate a thumbnail of an exact size, pass the desired
    /// size and then scale to the final size using a traditional
    /// resampling algorithm.
    ///
    /// The size of the image to be loaded, with the scale factor
    /// applied, is returned.
    pub fn scale(
        &mut self,
        requested_width: u16,
        requested_height: u16,
    ) -> ImageResult<(u16, u16)> {
        let result = self
            .decoder
            .scale(requested_width, requested_height)
            .map_err(jpeg_error_to_image_error)?;

        self.metadata.width = result.0;
        self.metadata.height = result.1;

        Ok(result)
    }
}

/// Wrapper struct around a `Cursor<Vec<u8>>`
pub struct JpegReader<R>(Cursor<Vec<u8>>, PhantomData<R>);
impl<R> Read for JpegReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        if self.0.position() == 0 && buf.is_empty() {
            mem::swap(buf, self.0.get_mut());
            Ok(buf.len())
        } else {
            self.0.read_to_end(buf)
        }
    }
}

impl<'a, R: 'a + Read, W: Write> ImageDecoder<'a> for AppMarkerJpegDecoder<R, W> {
    type Reader = JpegReader<R>;

    fn dimensions(&self) -> (u32, u32) {
        (
            u32::from(self.metadata.width),
            u32::from(self.metadata.height),
        )
    }

    fn color_type(&self) -> ColorType {
        pixel_format_to_color_type(self.metadata.pixel_format)
    }

    fn into_reader(mut self) -> ImageResult<Self::Reader> {
        let mut data = self.decoder.decode().map_err(jpeg_error_to_image_error)?;
        data = match self.decoder.info().unwrap().pixel_format {
            self::PixelFormat::CMYK32 => cmyk_to_rgb(&data),
            _ => data,
        };

        Ok(JpegReader(Cursor::new(data), PhantomData))
    }

    fn read_image(mut self, buf: &mut [u8]) -> ImageResult<()> {
        assert_eq!(u64::try_from(buf.len()), Ok(self.total_bytes()));

        let mut data = self.decoder.decode().map_err(jpeg_error_to_image_error)?;
        data = match self.decoder.info().unwrap().pixel_format {
            self::PixelFormat::CMYK32 => cmyk_to_rgb(&data),
            _ => data,
        };

        buf.copy_from_slice(&data);
        Ok(())
    }
}

fn cmyk_to_rgb(input: &[u8]) -> Vec<u8> {
    let count = input.len() / 4;
    let mut output = vec![0; 3 * count];

    let in_pixels = input[..4 * count].chunks_exact(4);
    let out_pixels = output[..3 * count].chunks_exact_mut(3);

    for (pixel, outp) in in_pixels.zip(out_pixels) {
        let c = 255 - u16::from(pixel[0]);
        let m = 255 - u16::from(pixel[1]);
        let y = 255 - u16::from(pixel[2]);
        let k = 255 - u16::from(pixel[3]);
        // CMY -> RGB
        let r = (k * c) / 255;
        let g = (k * m) / 255;
        let b = (k * y) / 255;

        outp[0] = r as u8;
        outp[1] = g as u8;
        outp[2] = b as u8;
    }

    output
}

fn pixel_format_to_color_type(pixel_format: self::PixelFormat) -> ColorType {
    use self::PixelFormat::*;
    match pixel_format {
        L8 => ColorType::L8,
        RGB24 => ColorType::Rgb8,
        CMYK32 => panic!(),
    }
}

fn jpeg_error_to_image_error(err: self::Error) -> ImageError {
    use self::Error::*;
    match err {
        err @ Format(_) => ImageError::Decoding(DecodingError::new(ImageFormat::Jpeg.into(), err)),
        Unsupported(desc) => ImageError::Unsupported(UnsupportedError::from_format_and_kind(
            ImageFormat::Jpeg.into(),
            UnsupportedErrorKind::GenericFeature(format!("{:?}", desc)),
        )),
        Io(err) => ImageError::IoError(err),
        Internal(err) => ImageError::Decoding(DecodingError::new(ImageFormat::Jpeg.into(), err)),
    }
}
