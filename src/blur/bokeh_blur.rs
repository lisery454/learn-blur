use crate::models::{color::Color, image::Image};

fn color_to_vec(c: Color) -> [f32; 3] {
    [c.r as f32, c.g as f32, c.b as f32]
}

fn vec_to_color(v: [f32; 3]) -> Color {
    Color {
        r: v[0].min(255.0).max(0.0) as u8,
        g: v[1].min(255.0).max(0.0) as u8,
        b: v[2].min(255.0).max(0.0) as u8,
    }
}

pub fn bokeh_blur(input: &Image, radius: f32, iterations: usize) -> Image {
    let width = input.width as i32;
    let height = input.height as i32;

    // 黄金角旋转（约 137.5 度）
    let golden_angle = 2.39996323_f32;
    let rot = |x: f32, y: f32| -> (f32, f32) {
        let cos_a = golden_angle.cos();
        let sin_a = golden_angle.sin();
        (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
    };

    let mut output = input.clone();

    for y in 0..height {
        for x in 0..width {
            let mut acc = [0.0_f32; 3];
            let mut divisor = [0.0_f32; 3];

            let mut r = 1.0_f32;
            let mut angle = (0.0_f32, radius);

            for _ in 0..iterations {
                r += 1.0 / r;
                angle = rot(angle.0, angle.1);

                let nx = x as f32 + (r - 1.0) * angle.0;
                let ny = y as f32 + (r - 1.0) * angle.1;

                let ix = nx.round() as i32;
                let iy = ny.round() as i32;

                if ix >= 0 && ix < width && iy >= 0 && iy < height {
                    let sample = input.pixels[iy as usize][ix as usize];
                    let sample_f = color_to_vec(sample);

                    for i in 0..3 {
                        // acc[i] += sample_f[i] * sample_f[i];
                        // divisor[i] += sample_f[i];
                        acc[i] += sample_f[i];
                        divisor[i] += 1.0;
                    }
                }
            }

            let mut final_color = [0.0_f32; 3];
            for i in 0..3 {
                final_color[i] = if divisor[i] != 0.0 {
                    acc[i] / divisor[i]
                } else {
                    0.0
                };
            }

            output.pixels[y as usize][x as usize] = vec_to_color(final_color);
        }
    }

    output
}
