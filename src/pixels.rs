use colorgrad::Color;
use image::{DynamicImage, GenericImageView, Rgba};

use crate::colour::Colour;
use crate::quantiser::Quantiser;

fn pixel_to_colour(pixel: Rgba<u8>) -> Colour {
    let [red, green, blue, _alpha] = pixel.0;

    Colour {
        red: red as usize,
        green: green as usize,
        blue: blue as usize,
    }
}

pub fn generate_pixel_vector(img: &DynamicImage, led_length: usize) -> Vec<Colour> {
    let mut quantiser = Quantiser::default();

    for (_x, _y, pixel) in img.pixels() {
        // TODO - find out if I need to convert alpha channel
        let colour = pixel_to_colour(pixel);

        quantiser.add_colour(&colour);
    }

    let colour_count = led_length / 2;
    let palette = quantiser.make_palette(led_length * 5);

    let gradient = colorgrad::CustomGradient::new()
        .colors(
            &palette
                .iter()
                .map(|c| Color::from_rgba8(c.red as u8, c.green as u8, c.blue as u8, 0))
                .collect::<Vec<_>>(),
        )
        .build()
        .unwrap();

    let line = gradient
        .colors(colour_count)
        .iter()
        .map(|c| Colour::from_rgba8(c.to_rgba8()))
        .collect::<Vec<_>>();

    line
}

pub fn morph(start: &[Colour], finish: &[Colour], steps: usize) -> Vec<Vec<Colour>> {
    let pixel_count = start.len();
    let targets = start.iter().zip(finish.iter());

    let mut morphs = Vec::with_capacity(steps);

    for _ in 0..steps {
        morphs.push(Vec::with_capacity(pixel_count));
    }

    for pixels in targets {
        let gradient = colorgrad::CustomGradient::new()
            .colors(&[
                Color::from_rgba8(
                    pixels.0.red as u8,
                    pixels.0.green as u8,
                    pixels.0.blue as u8,
                    0,
                ),
                Color::from_rgba8(
                    pixels.1.red as u8,
                    pixels.1.green as u8,
                    pixels.1.blue as u8,
                    0,
                ),
            ])
            .build()
            .unwrap();

        for (row, c) in gradient.colors(steps).iter().enumerate() {
            let morph = morphs.get_mut(row).unwrap();

            morph.push(Colour::from_rgba8(c.to_rgba8()));
        }
    }

    morphs
}
