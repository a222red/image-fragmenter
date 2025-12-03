mod args;
mod image_buf;

use crate::{
    args::Args,
    image_buf::{ImageBuf, get_random_span, random_transpose},
};
use anyhow::{Context, Result};
use gif::{Encoder, Frame};
use std::{fs::File, path::PathBuf};

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let mut source = ImageBuf::decode(&args.input).with_context(|| {
        format!(
            "Couldn't open input file '{}'",
            args.input.as_os_str().to_string_lossy()
        )
    })?;

    let mut work_buf = source.clone();

    let output_path = if let Some(path) = args.output {
        path
    } else {
        let mut path = PathBuf::from(args.input.file_stem().unwrap());
        path.set_extension("gif");

        path
    };

    let mut output = File::create(&output_path).with_context(|| {
        format!(
            "Couldn't create output file '{}'",
            output_path.as_os_str().to_string_lossy()
        )
    })?;
    let mut encoder = Encoder::new(&mut output, source.width() as _, source.height() as _, &[])?;

    let mut frame = Frame::from_rgb_speed(
        source.width() as _,
        source.height() as _,
        source.as_slice(),
        args.gif_encode_speed,
    );
    frame.delay = args.gif_frame_delay;

    encoder.write_frame(&frame)?;

    for i in 0..args.num_iterations {
        let span = get_random_span(
            source.width(),
            source.height(),
            args.chunk_min_width,
            args.chunk_min_height,
        );
        let dest = random_transpose(source.width(), source.height(), &span);

        work_buf.copy_chunk(&source, &span, dest);

        if args.recursive {
            source = work_buf.clone();
        }

        let mut frame = Frame::from_rgb_speed(
            source.width() as _,
            source.height() as _,
            work_buf.as_slice(),
            args.gif_encode_speed,
        );
        frame.delay = args.gif_frame_delay;

        encoder.write_frame(&frame)?;

        if let Some(fmt) = args.frame_export_format {
            let mut output_path = output_path.clone();
            output_path.set_extension(format!("{}.{}", (i + 1), fmt.extensions_str()[0]));

            work_buf.encode(&output_path, fmt)?;
        }
    }

    Ok(())
}
