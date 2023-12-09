mod colour;
mod node;
mod quantiser;

use colour::Colour;
use image::{GenericImageView, ImageBuffer, Rgb, RgbImage, Rgba};
use quantiser::Quantiser;

pub const MAX_DEPTH: u8 = 8;

fn pixel_to_colour(pixel: Rgba<u8>) -> Colour {
    let [red, green, blue, _alpha] = pixel.0;

    Colour {
        red: red as usize,
        green: green as usize,
        blue: blue as usize,
    }
}

fn main() {
    let path = "art/baynk.jpeg";

    let img = image::open(path).unwrap();

    let mut quantiser = Quantiser::default();

    for (_x, _y, pixel) in img.pixels() {
        // TODO - find out if I need to convert alpha channel
        let colour = pixel_to_colour(pixel);

        quantiser.add_colour(&colour);
    }

    let width = 16;
    let height = 16;

    let palette = quantiser.make_palette(width * height);

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    for (index, colour) in palette.iter().enumerate() {
        let x = (index % height) as u32;
        let y = (index / height) as u32;

        img.put_pixel(
            x,
            y,
            Rgb([colour.red as u8, colour.green as u8, colour.blue as u8]),
        );
    }

    img.save("sample.png").unwrap();
}
