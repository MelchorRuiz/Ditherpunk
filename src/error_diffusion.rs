use std::vec;
use image::io::Reader as ImageReader;
use image::Rgb;
use css_color_parser::Color as CssColor;

pub fn apply_error_diffusion(input: String, output: Option<String>, algorithm: String) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input)?.decode()?;
    let mut img_rgb = img.to_rgb8();

    let width = img_rgb.width();
    let height = img_rgb.height();

    let colors = vec![
        "black".parse::<CssColor>().unwrap(),
        "white".parse::<CssColor>().unwrap(),
        "red".parse::<CssColor>().unwrap(),
        "green".parse::<CssColor>().unwrap(),
        "blue".parse::<CssColor>().unwrap(),
        "yellow".parse::<CssColor>().unwrap(),
        "cyan".parse::<CssColor>().unwrap(),
        "magenta".parse::<CssColor>().unwrap(),
    ];

    for x in 0..width {
        for y in 0..height {
            let pixel = img_rgb.get_pixel_mut(x, y);
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

            let error_r = r - closest_color.r as f32;
            let error_g = g - closest_color.g as f32;
            let error_b = b - closest_color.b as f32;

            match algorithm.to_lowercase().as_str() {
                "floyd_steinberg" => {
                    let diffusion_matrix: [[f32; 3]; 3] = [
                        [0.0, 0.0, 7.0],
                        [3.0, 5.0, 1.0],
                        [0.0, 0.0, 0.0],
                    ];
                    let divisor = 16.0;

                    for dy in 0..2 {
                        for dx in 0..3 {
                            if diffusion_matrix[dy][dx] == 0.0 {
                                continue;
                            }
                            let nx = x as isize + dx as isize - 1;
                            let ny = y as isize + dy as isize;
                            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                                let pixel = img_rgb.get_pixel_mut(nx as u32, ny as u32);
                                pixel[0] = (pixel[0] as f32 + error_r * diffusion_matrix[dy][dx] / divisor).round() as u8;
                                pixel[1] = (pixel[1] as f32 + error_g * diffusion_matrix[dy][dx] / divisor).round() as u8;
                                pixel[2] = (pixel[2] as f32 + error_b * diffusion_matrix[dy][dx] / divisor).round() as u8;
                            }
                        }
                    }
                },
                "jarvis_judice_ninke" => {
                    let diffusion_matrix: [[f32; 5]; 3] = [
                        [0.0, 0.0, 0.0, 7.0, 5.0],
                        [3.0, 5.0, 7.0, 5.0, 3.0],
                        [1.0, 3.0, 5.0, 3.0, 1.0],
                    ];
                    let divisor = 48.0;

                    for dy in 0..3 {
                        for dx in 0..5 {
                            if diffusion_matrix[dy][dx] == 0.0 {
                                continue;
                            }
                            let nx = x as isize + dx as isize - 2;
                            let ny = y as isize + dy as isize;
                            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                                let pixel = img_rgb.get_pixel_mut(nx as u32, ny as u32);
                                pixel[0] = (pixel[0] as f32 + error_r * diffusion_matrix[dy][dx] / divisor).round() as u8;
                                pixel[1] = (pixel[1] as f32 + error_g * diffusion_matrix[dy][dx] / divisor).round() as u8;
                                pixel[2] = (pixel[2] as f32 + error_b * diffusion_matrix[dy][dx] / divisor).round() as u8;
                            }
                        }
                    }
                },
                "atkinson" => {
                    let diffusion_matrix: [[f32; 5]; 3] = [
                        [0.0, 0.0, 1.0, 1.0, 0.0],
                        [0.0, 1.0, 1.0, 1.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0, 0.0],
                    ];
                    let divisor = 8.0;

                    for dy in 0..3 {
                        for dx in 0..5 {
                            if diffusion_matrix[dy][dx] == 0.0 {
                                continue;
                            }
                            let nx = x as isize + dx as isize - 2;
                            let ny = y as isize + dy as isize;
                            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                                let pixel = img_rgb.get_pixel_mut(nx as u32, ny as u32);
                                pixel[0] = (pixel[0] as f32 + error_r * diffusion_matrix[dy][dx] / divisor).round() as u8;
                                pixel[1] = (pixel[1] as f32 + error_g * diffusion_matrix[dy][dx] / divisor).round() as u8;
                                pixel[2] = (pixel[2] as f32 + error_b * diffusion_matrix[dy][dx] / divisor).round() as u8;
                            }
                        }
                    }
                },
                _ => {
                    return Err("L’algorithme de diffusion d’erreur doit être dans la liste [FLOYD_STEINBERG, JARVIS_JUDICE_NINKE, ATKINSON].".into());
                },
            }
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
    fn test_apply_error_diffusion() {
        let input = String::from("img/testing/1.jpg");
        let expected = String::from("img/testing/7.jpg");
        let output = String::from("img/testing/output-7.jpg");

        let r = apply_error_diffusion(input, Some(output.clone()), "floyd_steinberg".to_string());
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