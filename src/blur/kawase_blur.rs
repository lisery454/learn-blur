use crate::models::{color::Color, image::Image};

pub fn kawase_blur(image: &Image, count: u32) -> Image {
    let mut result = image.clone();
    for i in 1..=count {
        result = kawase_pass(&result, i); // offset 随着 pass 增大
    }
    result
}

fn kawase_pass(input: &Image, offset: u32) -> Image {
    let width = input.width;
    let height = input.height;

    let mut pixels: Vec<Vec<Color>> =
        vec![vec![Color::new(0, 0, 0); width as usize]; height as usize];

    let offset = offset as i32;

    for y in 0..height {
        for x in 0..width {
            let mut r = 0u32;
            let mut g = 0u32;
            let mut b = 0u32;

            let mut count = 0u32;

            // 四个对角方向
            let offsets = [
                (offset, offset),
                (-offset, offset),
                (offset, -offset),
                (-offset, -offset),
            ];

            for (dx, dy) in offsets {
                let nx = (x as i32 + dx).clamp(0, (width - 1) as i32) as u32;
                let ny = (y as i32 + dy).clamp(0, (height - 1) as i32) as u32;

                let pixel = input.get_pixel(nx, ny);
                r += pixel.r as u32;
                g += pixel.g as u32;
                b += pixel.b as u32;

                count += 1;
            }

            let x = x as usize;
            let y = y as usize;

            pixels[y][x] = Color::new((r / count) as u8, (g / count) as u8, (b / count) as u8);
        }
    }

    let output = Image::new(width, height, pixels);

    output
}
