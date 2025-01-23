mod args;
mod white_pixel_alternation;
mod thresholding;
mod pallet;
mod dithering;
mod ordered_dithering;
mod error_diffusion;

use args::{DitherArgs, Mode};
use white_pixel_alternation::apply_white_pixel_alternation;
use thresholding::apply_thresholding;
use pallet::apply_pallet;
use dithering::apply_dithering;
use ordered_dithering::apply_ordered_dithering;
use error_diffusion::apply_error_diffusion;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: DitherArgs = argh::from_env();

    match args.mode {
        Mode::WhitePixelAlternation(_) => {
            apply_white_pixel_alternation(args.input, args.output)?;
        },
        Mode::Thresholding(opts) => {
            apply_thresholding(args.input, args.output, opts.light, opts.dark)?;
        },
        Mode::Pallet(opts) => {
            apply_pallet(args.input, args.output, opts.n)?;
        },
        Mode::Dithering(_) => {
            apply_dithering(args.input, args.output)?;
        },
        Mode::OrderedDithering(opts) => {
            apply_ordered_dithering(args.input, args.output, opts.n)?;
        },
        Mode::ErrorDiffusion(opts) => {
            apply_error_diffusion(args.input, args.output, opts.algorithm)?;
        },
    }

    Ok(())
}
