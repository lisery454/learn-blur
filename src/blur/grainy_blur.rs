use rand::Rng;

use crate::models::image::Image;

pub fn grainy_blur(input: &Image, offset: u32) -> Image {
    let mut rng = rand::rng();

    let mut result = input.clone();

    for x in 0..input.width {
        for y in 0..input.height {
            let min_x = 0.max(x as i32 - offset as i32) as u32;
            let max_x = (input.width - 1).min(x + offset);
            let min_y = 0.max(y as i32 - offset as i32) as u32;
            let max_y = (input.height - 1).min(y + offset);

            let nx = rng.random_range(min_x..=max_x);
            let ny = rng.random_range(min_y..=max_y);

            let color = input.get_pixel(nx, ny);
            result.pixels[y as usize][x as usize] = color;
        }
    }

    result
}
