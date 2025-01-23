use image::io::Reader as ImageReader;
use image::Rgb;

pub fn apply_ordered_dithering(input: String, output: Option<String>, n: u32) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input)?.decode()?;
    let mut img_rgb = img.to_rgb8();

    let bayer_matrix = generate_bayer_matrix(n);

    for (x, y, pixel) in img_rgb.enumerate_pixels_mut() {
        let (r, g, b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);

        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let threshold = bayer_matrix[(x as usize) % bayer_matrix.len()][(y as usize) % bayer_matrix.len()];

        let new_pixel = if luminance > threshold * 255 as f32 {
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

fn generate_bayer_matrix(n: u32) -> Vec<Vec<f32>> {
    let size = 2_usize.pow(n);
    let mut matrix = vec![vec![0_u32; size]; size];

    fn fill_bayer(matrix: &mut Vec<Vec<u32>>, x: usize, y: usize, size: usize, value: u32) {
        if size == 1 {
            matrix[x][y] = value;
            return;
        }

        let half = size / 2;
        let factor = (size * size / 4) as u32;

        fill_bayer(matrix, x, y, half, value + factor * 0);
        fill_bayer(matrix, x + half, y, half, value + factor * 3);
        fill_bayer(matrix, x, y + half, half, value + factor * 2);
        fill_bayer(matrix, x + half, y + half, half, value + factor * 1);
    }

    fill_bayer(&mut matrix, 0, 0, size, 0);

    let max_value = (size * size) as u32 - 1;
    let matrix = matrix.into_iter()
        .map(|row| row.into_iter().map(|value| value as f32 / max_value as f32).collect())
        .collect();

    matrix
}