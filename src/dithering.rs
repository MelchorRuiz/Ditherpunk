use image::io::Reader as ImageReader;
use image::Rgb;
use rand::Rng;

pub fn apply_dithering(input: String, output: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input)?.decode()?;
    let mut img_rgb = img.to_rgb8();

    let mut rng = rand::thread_rng();

    for (_, _, pixel) in img_rgb.enumerate_pixels_mut() {
        let (r, g, b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);

        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let threshold = rng.gen_range(0.0..1.0);
        let new_pixel = if luminance > threshold * 255.0 {
            Rgb([255, 255, 255])
        } else {
            Rgb([0, 0, 0])
        };

        *pixel = new_pixel;
    }

    match output {
        Some(output) => img_rgb.save(output)?,
        None => img_rgb.save("output.png")?,
    }

    Ok(())
}
