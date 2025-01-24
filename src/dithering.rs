use image::io::Reader as ImageReader;
use image::Rgb;
use rand::Rng;

pub fn apply_dithering<R: Rng>(input: String, output: Option<String>, rng: &mut R) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input)?.decode()?;
    let mut img_rgb = img.to_rgb8();


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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_apply_dithering() {
        let input = String::from("img/testing/1.jpg");
        let expected = String::from("img/testing/5.jpg");
        let output = String::from("img/testing/output-5.jpg");
        
        let mut rng = StdRng::seed_from_u64(42);
        let r = apply_dithering(input, Some(output.clone()), &mut rng);
        assert!(r.is_ok());

        let img_expected = ImageReader::open(expected).unwrap().decode().unwrap();
        let img_output = ImageReader::open(output.clone()).unwrap().decode().unwrap();
        let img_expected_rgb = img_expected.to_rgb8();
        let img_output_rgb = img_output.to_rgb8();

        for (x, y, pixel) in img_expected_rgb.enumerate_pixels() {
            assert_eq!(pixel, img_output_rgb.get_pixel(x, y));
        }

        std::fs::remove_file(output).unwrap();
    }
}