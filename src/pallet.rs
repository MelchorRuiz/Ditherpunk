use image::io::Reader as ImageReader;
use image::Rgb;
use css_color_parser::Color as CssColor;

pub fn apply_pallet(input: String, output: Option<String>, n: usize) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input)?.decode()?;
    let mut img_rgb = img.to_rgb8();

    if (n <= 0) || (n > 8) {
        return Err("Le nombre de couleurs doit être compris entre 2 et 8.".into());
    }

    let mut colors = vec![
        "black".parse::<CssColor>().unwrap(),
        "white".parse::<CssColor>().unwrap(),
        "red".parse::<CssColor>().unwrap(),
        "green".parse::<CssColor>().unwrap(),
        "blue".parse::<CssColor>().unwrap(),
        "yellow".parse::<CssColor>().unwrap(),
        "cyan".parse::<CssColor>().unwrap(),
        "magenta".parse::<CssColor>().unwrap(),
    ];

    colors.truncate(n);

    for (_, _, pixel) in img_rgb.enumerate_pixels_mut() {
        let (r, g, b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);

        let mut min_distance = f32::MAX;
        let mut closest_color = &colors[0];

        for color in &colors {
            let dr = r - color.r as f32;
            let dg = g - color.g as f32;
            let db = b - color.b as f32;

            let distance = dr * dr + dg * dg + db * db;

            if distance < min_distance {
                min_distance = distance;
                closest_color = color;
            }
        }

        *pixel = Rgb([closest_color.r, closest_color.g, closest_color.b]);
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
    fn test_apply_pallet() {
        let input = String::from("img/testing/1.jpg");
        let expected = String::from("img/testing/4.jpg");
        let output = String::from("img/testing/output-4.jpg");

        let r = apply_pallet(input, Some(output.clone()), 8);
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