use crate::models::{color::Color, image::Image};

#[allow(dead_code)]
pub fn box_blur(image: &Image, size: u8) -> Image {
    let size = size as i32;
    let mut new_pixels =
        vec![vec![Color { r: 0, g: 0, b: 0 }; image.width as usize]; image.height as usize];

    let half = size / 2;

    for y in 0..image.height as i32 {
        for x in 0..image.width as i32 {
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;
            let mut count = 0u32;

            // 遍历 box 区域
            for dy in -half..=half {
                for dx in -half..=half {
                    let nx = x + dx;
                    let ny = y + dy;

                    // 边界检查
                    if nx >= 0 && nx < image.width as i32 && ny >= 0 && ny < image.height as i32 {
                        let pixel = &image.pixels[ny as usize][nx as usize];
                        sum_r += pixel.r as u32;
                        sum_g += pixel.g as u32;
                        sum_b += pixel.b as u32;
                        count += 1;
                    }
                }
            }

            new_pixels[y as usize][x as usize] = Color {
                r: (sum_r / count) as u8,
                g: (sum_g / count) as u8,
                b: (sum_b / count) as u8,
            };
        }
    }

    Image::new(image.width, image.height, new_pixels)
}

pub fn box_blur_integral(image: &Image, size: u8) -> Image {
    let w = image.width as usize;
    let h = image.height as usize;
    if size & 1 != 1 {
        panic!("size is not odd!")
    }
    let half = (size / 2) as usize;

    let mut sat_r = vec![vec![0u32; w + 1]; h + 1];
    let mut sat_g = vec![vec![0u32; w + 1]; h + 1];
    let mut sat_b = vec![vec![0u32; w + 1]; h + 1];

    for y in 0..h {
        for x in 0..w {
            let c = &image.pixels[y][x];
            sat_r[y + 1][x + 1] = c.r as u32 + sat_r[y + 1][x] + sat_r[y][x + 1] - sat_r[y][x];
            sat_g[y + 1][x + 1] = c.g as u32 + sat_g[y + 1][x] + sat_g[y][x + 1] - sat_g[y][x];
            sat_b[y + 1][x + 1] = c.b as u32 + sat_b[y + 1][x] + sat_b[y][x + 1] - sat_b[y][x];
        }
    }

    let mut new_pixels = vec![vec![Color { r: 0, g: 0, b: 0 }; w]; h];

    for y in 0..h {
        for x in 0..w {
            // box 区域坐标
            let x1 = x.saturating_sub(half);
            let y1 = y.saturating_sub(half);
            let x2 = (x + half).min(w - 1);
            let y2 = (y + half).min(h - 1);

            let area = ((x2 - x1 + 1) * (y2 - y1 + 1)) as u32;

            // 利用积分图快速求和
            let sum_r =
                sat_r[y2 + 1][x2 + 1] + sat_r[y1][x1] - sat_r[y1][x2 + 1] - sat_r[y2 + 1][x1];
            let sum_g =
                sat_g[y2 + 1][x2 + 1] + sat_g[y1][x1] - sat_g[y1][x2 + 1] - sat_g[y2 + 1][x1];
            let sum_b =
                sat_b[y2 + 1][x2 + 1] + sat_b[y1][x1] - sat_b[y1][x2 + 1] - sat_b[y2 + 1][x1];

            new_pixels[y][x] = Color {
                r: (sum_r / area) as u8,
                g: (sum_g / area) as u8,
                b: (sum_b / area) as u8,
            };
        }
    }

    Image::new(image.width, image.height, new_pixels)
}
