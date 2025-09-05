use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum BlurType {
    Gaussian {
        #[arg(short, long, help = "模糊采样的半径长度", value_parser = validate_radius)]
        radius: u8,

        #[arg(short, long, help = "高斯函数的sigma值，默认为radius / 2")]
        sigma: Option<f64>,
    },
    Box {
        #[arg(short, long, help = "box的边长")]
        size: u8,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct BlurArgs {
    #[arg(short, long, help = "指定要模糊的图片文件")]
    pub input: String,

    #[arg(
        short,
        long,
        default_value = "output.png",
        help = "指定的输出图片文件名称"
    )]
    pub output: String,

    #[command(subcommand)]
    pub blur_type: BlurType,

    #[arg(short, long, default_value_t = 1, help = "模糊迭代次数")]
    pub count: u32,

    #[arg(short,long, default_value_t = 1_f64, value_parser = validate_downrate, help="模糊前缩放比例")]
    pub downrate: f64,
}

fn validate_downrate(s: &str) -> Result<f64, String> {
    match s.parse::<f64>() {
        Ok(val) if val > 0.0 && val <= 1.0 => Ok(val),
        _ => Err(String::from("downrate 必须大于 0 且小于等于 1")),
    }
}

fn validate_radius(s: &str) -> Result<u8, String> {
    match s.parse::<u8>() {
        Ok(val) if val > 0 => Ok(val),
        _ => Err(String::from("radius 必须大于 0 ")),
    }
}
