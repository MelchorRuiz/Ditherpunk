mod args;
use image::ImageError;
mod white_pixel_alternation;

use args::{DitherArgs, Mode};
use white_pixel_alternation::apply_white_pixel_alternation;

fn main() -> Result<(), ImageError> {
    let args: DitherArgs = argh::from_env();

    match args.mode {
        Mode::WhitePixelAlternation(_) => {
            apply_white_pixel_alternation(args.input, args.output)?;
        }
    }

    Ok(())
}
