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
