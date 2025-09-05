use crate::models::{color::Color, image::Image};

pub fn gaussian_blur(image: &Image, radius: u8, sigma: f64) -> Image {
    let w = image.width as usize;
    let h = image.height as usize;
    let radius = radius as usize;

    // generate kernel
    let mut kernel = Vec::with_capacity(radius * 2 + 1);
    let mut sum = 0.0;
    for i in 0..=2 * radius {
        let x = i as i32 - radius as i32;
        let val = (-(x * x) as f64 / (2.0 * sigma * sigma)).exp();
        kernel.push(val);
        sum += val;
    }
    // normalize
    for k in &mut kernel {
        *k /= sum;
    }

    // horizonal
    let mut horiz = vec![vec![Color { r: 0, g: 0, b: 0 }; w]; h];
    for y in 0..h {
        for x in 0..w {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            for k in 0..kernel.len() {
                let xi = x as isize + k as isize - radius as isize;
                let xi = xi.clamp(0, (w - 1) as isize) as usize;
                let c = &image.pixels[y][xi];
                r += c.r as f64 * kernel[k];
                g += c.g as f64 * kernel[k];
                b += c.b as f64 * kernel[k];
            }
            horiz[y][x] = Color {
                r: r.round() as u8,
                g: g.round() as u8,
                b: b.round() as u8,
            };
        }
    }

    // vertical
    let mut result = vec![vec![Color { r: 0, g: 0, b: 0 }; w]; h];
    for x in 0..w {
        for y in 0..h {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            for k in 0..kernel.len() {
                let yi = y as isize + k as isize - radius as isize;
                let yi = yi.clamp(0, (h - 1) as isize) as usize;
                let c = &horiz[yi][x];
                r += c.r as f64 * kernel[k];
                g += c.g as f64 * kernel[k];
                b += c.b as f64 * kernel[k];
            }
            result[y][x] = Color {
                r: r.round() as u8,
                g: g.round() as u8,
                b: b.round() as u8,
            };
        }
    }

    Image::new(image.width, image.height, result)
}
