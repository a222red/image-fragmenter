use argh::FromArgs;
use image::ImageFormat;
use std::path::PathBuf;

#[derive(FromArgs)]
/// Composite an image by repeatedly layering chunks over itself
pub struct Args {
    /// input file (must be in BMP format)
    #[argh(positional)]
    pub input: PathBuf,

    /// output file
    #[argh(option, short = 'o')]
    pub output: Option<PathBuf>,

    /// number of iterations
    #[argh(option, short = 'n')]
    pub num_iterations: u32,

    /// copy chunks from the previous iteration's output instead of from the original image
    #[argh(switch, short = 'r')]
    pub recursive: bool,

    /// minimum chunk width in pixels
    #[argh(option, short = 'w', default = "16")]
    pub chunk_min_width: u32,

    /// minimum chunk width in pixels
    #[argh(option, short = 'h', default = "16")]
    pub chunk_min_height: u32,

    /// gif encoding speed, between 1 and 30
    #[argh(option, short = 's', default = "10")]
    pub gif_encode_speed: i32,

    /// gif frame delay in increments of 10ms
    #[argh(option, short = 'd', default = "10")]
    pub gif_frame_delay: u16,

    /// export each frame as an individual image
    #[argh(option, short = 'f', from_str_fn(str_to_img_fmt))]
    pub frame_export_format: Option<ImageFormat>,
}

//TODO: List supported image formats
fn str_to_img_fmt(str: &str) -> Result<ImageFormat, String> {
    ImageFormat::from_extension(str).ok_or_else(|| format!("Unrecognized image format: '{}'", str))
}
