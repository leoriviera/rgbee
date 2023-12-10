mod colour;
mod node;
mod quantiser;

use colorgrad::Color;
use colour::Colour;
use image::{GenericImageView, ImageBuffer, Rgb, RgbImage, Rgba};
use quantiser::Quantiser;
use std::fs;

pub const MAX_DEPTH: u8 = 8;

fn pixel_to_colour(pixel: Rgba<u8>) -> Colour {
    let [red, green, blue, _alpha] = pixel.0;

    Colour {
        red: red as usize,
        green: green as usize,
        blue: blue as usize,
    }
}

fn generate_pixel_vector(path: &str, led_length: usize) -> Vec<Colour> {
    let img = image::open(path).unwrap();

    let mut quantiser = Quantiser::default();

    for (_x, _y, pixel) in img.pixels() {
        // TODO - find out if I need to convert alpha channel
        let colour = pixel_to_colour(pixel);

        quantiser.add_colour(&colour);
    }

    let colour_count = led_length / 2;
    let palette = quantiser.make_palette(colour_count);

    let gradient = colorgrad::CustomGradient::new()
        .colors(
            &palette
                .iter()
                .map(|c| Color::from_rgba8(c.red as u8, c.green as u8, c.blue as u8, 0))
                .collect::<Vec<_>>(),
        )
        .domain(&[0.0, colour_count as f64])
        .build()
        .unwrap();

    let mut line: Vec<Colour> = Vec::new();

    for point in (0..colour_count).map(|n| n as f64) {
        let gradient_point = gradient.at(point).to_rgba8();
        line.push(Colour::from_rgba8(gradient_point));
    }

    line.extend(line.clone().into_iter().rev());

    line
}

fn main() {
    let led_length = 300;

    let paths = fs::read_dir("samples").unwrap();

    for path in paths {
        let p = path.unwrap();

        let artwork_path = p.path().into_os_string().into_string().unwrap();
        let file = p.file_name().into_string().unwrap();

        let line = generate_pixel_vector(&artwork_path, led_length);

        let mut img: RgbImage = ImageBuffer::new(led_length as u32, 1);

        for (index, colour) in line.iter().enumerate() {
            img.put_pixel(
                index as u32,
                0,
                Rgb([colour.red as u8, colour.green as u8, colour.blue as u8]),
            );
        }

        img.save(format!("output/{file}")).unwrap();
    }
}
