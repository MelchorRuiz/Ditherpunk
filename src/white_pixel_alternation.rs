use image::ImageError;
use image::Rgb;
use image::io::Reader as ImageReader;

pub fn apply_white_pixel_alternation(input: String, output: Option<String>) -> Result<(), ImageError> {
    let img = ImageReader::open(input)?.decode()?;
    println!("type: {:?}", img.color());
    let mut img_rgb = img.to_rgb8(); 
    // img_rgb.save("img/gato_rgb.png")?;

    println!("(32, 52) {:?}", img_rgb.get_pixel(32, 52));
    
    for (x, y, pixel) in img_rgb.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = Rgb([255, 255, 255]);
        }
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

    #[test]
    fn test_apply_white_pixel_alternation() {
        let input = String::from("img/testing/1.jpg");
        let expected = String::from("img/testing/2.jpg");
        let output = String::from("img/testing/output-2.jpg");

        let r = apply_white_pixel_alternation(input, Some(output.clone()));
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