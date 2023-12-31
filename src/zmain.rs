mod colour;
mod node;
mod quantiser;

use colorgrad::Color;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};

use colour::Colour;
use quantiser::Quantiser;

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

    let mut line = gradient
        .colors(colour_count)
        .iter()
        .map(|c| {
            let rgb = c.to_rgba8();

            Colour {
                red: rgb[0] as usize,
                green: rgb[1] as usize,
                blue: rgb[2] as usize,
            }
        })
        .collect::<Vec<_>>();

    line.extend(line.clone().into_iter().rev());

    line
}

fn main() {
    const PIN: i32 = 10;
    const LED_LENGTH: usize = 82;

    let artwork_path = "samples/bombalaya.jpeg";
    let img = image::open(artwork_path).unwrap();

    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(PIN) // GPIO 10 = SPI0 MOSI
                .count(LED_LENGTH as i32) // Number of LEDs
                .strip_type(StripType::Ws2812)
                .brightness(15) // default: 255
                .build(),
        )
        .build()
        .unwrap();
    let leds = controller.leds_mut(0);

    let v = generate_pixel_vector(&img, LED_LENGTH);

    for (i, led) in leds.iter_mut().enumerate() {
        let c = v.get(i).unwrap();
        dbg!(c);
        // LEDs are BGRA
        *led = [c.blue as u8, c.green as u8, c.red as u8, 0];
    }

    controller.render().unwrap();

    // let mut output_img = ImageBuffer::new(LED_LENGTH as u32, 100);

    // for (index, colour) in v.iter().enumerate() {
    //     output_img.put_pixel(index as u32, 0, Rgba(*colour));
    // }

    // output_img.save("output/output.jpeg").unwrap();
}
