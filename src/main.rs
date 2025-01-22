mod args;
mod white_pixel_alternation;
mod thresholding;

use args::{DitherArgs, Mode};
use white_pixel_alternation::apply_white_pixel_alternation;
use thresholding::apply_thresholding;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: DitherArgs = argh::from_env();

    match args.mode {
        Mode::WhitePixelAlternation(_) => {
            apply_white_pixel_alternation(args.input, args.output)?;
        },
        Mode::Thresholding(opts) => {
            apply_thresholding(args.input, args.output, opts.light, opts.dark)?;
        },
    }

    Ok(())
}
