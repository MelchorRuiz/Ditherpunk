use image::ImageError;
use image::Rgb;
use image::io::Reader as ImageReader;

fn main() -> Result<(), ImageError> {
    let img = ImageReader::open("img/gato.png")?.decode()?;
    println!("type: {:?}", img.color());
    let mut img_rgb = img.to_rgb8(); 
    img_rgb.save("img/gato_rgb.png")?;

    println!("(32, 52) {:?}", img_rgb.get_pixel(32, 52));
    
    for (x, y, pixel) in img_rgb.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = Rgb([255, 255, 255]);
        }
    }

    img_rgb.save("img/gato_rgb_white.png")?;

    Ok(())
}
