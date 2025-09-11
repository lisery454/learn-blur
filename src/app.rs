use std::time::Instant;

use clap::Parser;

use crate::{
    args::BlurArgs,
    blur::{
        bokeh_blur::bokeh_blur, box_blur::box_blur_integral, gaussian_blur::gaussian_blur,
        grainy_blur::grainy_blur, kawase_blur::kawase_blur,
    },
    models::image::Image,
};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let start_time = Instant::now();
        let args = BlurArgs::parse();
        let mut image = Image::read_from(&args.input);

        let scale = args.downrate;
        let original_width = image.width;
        let original_height = image.height;
        let small_width = ((image.width as f64 * scale) as u32).max(1);
        let small_height = ((image.height as f64 * scale) as u32).max(1);
        image.scale(small_width as usize, small_height as usize);

        let mut output_image = image;
        for _ in 0..args.count {
            output_image = match args.blur_type {
                crate::args::BlurType::Gaussian { radius, sigma } => {
                    let sigma = match sigma {
                        Some(value) => value,
                        None => radius as f64 / 2.0,
                    };
                    gaussian_blur(&output_image, radius, sigma)
                }
                crate::args::BlurType::Box { size } => box_blur_integral(&output_image, size),
                crate::args::BlurType::Kawase { kawase_count } => {
                    kawase_blur(&output_image, kawase_count)
                }
                crate::args::BlurType::Bokeh { radius, iter_count } => {
                    bokeh_blur(&output_image, radius, iter_count.into())
                }
                crate::args::BlurType::Grainy { offset } => grainy_blur(&output_image, offset),
            };
        }
        output_image.scale(original_width as usize, original_height as usize);

        output_image.write_to(&args.output);

        println!("use time: {:?}", start_time.elapsed());
    }
}
