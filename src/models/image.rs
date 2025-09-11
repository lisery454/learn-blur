use image::{Rgb, RgbImage};

use crate::models::color::Color;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Color>>,
}

impl Image {
    pub fn new(width: u32, height: u32, pixels: Vec<Vec<Color>>) -> Self {
        Self {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let x = x as usize;
        let y = y as usize;
        return self.pixels[y][x];
    }

    pub fn write_to(&self, path: &String) {
        let mut img = RgbImage::new(self.width, self.height);

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let c = &self.pixels[y][x];
                img.put_pixel(x as u32, y as u32, Rgb([c.r, c.g, c.b]));
            }
        }

        img.save(path).expect("save image failed.");
    }

    pub fn read_from(path: &String) -> Self {
        let img = image::open(path).unwrap().to_rgb8();
        let (width, height) = img.dimensions();

        let mut pixels: Vec<Vec<Color>> =
            vec![vec![Color::new(0, 0, 0); width as usize]; height as usize];

        for y in 0..height as usize {
            for x in 0..width as usize {
                let pixel = img.get_pixel(x as u32, y as u32);
                pixels[y][x] = Color::new(pixel.0[0], pixel.0[1], pixel.0[2]);
            }
        }

        Self::new(width, height, pixels)
    }

    pub fn scale(&mut self, target_width: usize, target_height: usize) {
        let mut pixels = vec![vec![Color { r: 0, g: 0, b: 0 }; target_width]; target_height];

        for y in 0..target_height {
            for x in 0..target_width {
                let orig_x = x * self.width as usize / target_width;
                let orig_y = y * self.height as usize / target_height;
                pixels[y][x] = self.pixels[orig_y][orig_x].clone();
            }
        }

        self.height = target_height as u32;
        self.width = target_width as u32;
        self.pixels = pixels;
    }
}
