use image::io::Reader as ImageReader;
use image::Rgb;
use css_color_parser::Color as CssColor;

pub fn apply_umbralization(input: String, output: Option<String>, light: Option<String>, dark: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input)?.decode()?;
    let mut img_rgb = img.to_rgb8();

    let light = light.unwrap_or(String::from("#FFFFFF"));
    let dark = dark.unwrap_or(String::from("#000000"));
    let color_light = light.as_str().parse::<CssColor>()?;
    let color_dark = dark.as_str().parse::<CssColor>()?;

    for (_, _, pixel) in img_rgb.enumerate_pixels_mut() {
        let (r, g, b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);

        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let new_pixel = if luminance > 128.0 {
            Rgb([color_light.r, color_light.g, color_light.b])
        } else {
            Rgb([color_dark.r, color_dark.g, color_dark.b])
        };

        *pixel = new_pixel;
    }

    match output {
        Some(output) => img_rgb.save(output)?,
        None => img_rgb.save("output.png")?,
    }

    Ok(())
}
