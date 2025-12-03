use anyhow::Result;
use image::{DynamicImage, ImageFormat, ImageReader, RgbImage};
use rand::random_range;
use std::{ops::Range, path::Path};

#[derive(Clone)]
pub struct ImageBuf {
    width: u32,
    height: u32,
    buf: Box<[u8]>,
}

impl ImageBuf {
    pub fn decode(path: &Path) -> Result<Self> {
        let img = ImageReader::open(path)?.decode()?.into_rgb8();

        let width = img.width();
        let height = img.height();

        let bytes = img.into_vec();

        assert_eq!(bytes.len() as u32, width * height * 3);

        Ok(Self {
            width,
            height,
            buf: bytes.into_boxed_slice(),
        })
    }

    pub fn encode(&self, path: &Path, format: ImageFormat) -> Result<()> {
        let img = RgbImage::from_raw(self.width, self.height, self.buf.to_vec()).unwrap();
        let img = DynamicImage::ImageRgb8(img);

        img.save_with_format(path, format)?;

        Ok(())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let i = (self.width * y + x) as usize;

        let r = self.buf[3 * i];
        let g = self.buf[3 * i + 1];
        let b = self.buf[3 * i + 2];

        Pixel { r, g, b }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, px: Pixel) {
        let i = (self.width * y + x) as usize;

        self.buf[3 * i] = px.r;
        self.buf[3 * i + 1] = px.g;
        self.buf[3 * i + 2] = px.b;
    }

    pub fn copy_chunk(&mut self, source: &ImageBuf, from: &Span, to: (u32, u32)) {
        for (i, y) in from.y.clone().enumerate() {
            for (j, x) in from.x.clone().enumerate() {
                let px = source.get_pixel(x, y);
                self.set_pixel(to.0 + j as u32, to.1 + i as u32, px);
            }
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}

#[derive(Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Span {
    x: Range<u32>,
    y: Range<u32>,
}

impl Span {
    pub fn width(&self) -> u32 {
        self.x.end - self.x.start
    }

    pub fn height(&self) -> u32 {
        self.y.end - self.y.start
    }
}

pub fn get_random_span(img_width: u32, img_height: u32, min_width: u32, min_height: u32) -> Span {
    let x_start = random_range(0..(img_width - min_width));
    let x_end = random_range((x_start + min_width)..img_width);

    let y_start = random_range(0..(img_height - min_height));
    let y_end = random_range((y_start + min_height)..img_height);

    Span {
        x: x_start..x_end,
        y: y_start..y_end,
    }
}

pub fn random_transpose(img_width: u32, img_height: u32, span: &Span) -> (u32, u32) {
    let x_start = random_range(0..(img_width - span.width()));
    let y_start = random_range(0..(img_height - span.height()));

    (x_start, y_start)
}
