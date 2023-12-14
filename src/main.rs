use rs_ws281x::ChannelBuilder;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;

mod colour;
mod node;
mod quantiser;

use colorgrad::Color;
use colour::Colour;
use image::{GenericImageView, Rgba};
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

fn generate_pixel_vector(path: &str, led_length: usize) -> Vec<[u8; 4]> {
    let img = image::open(path).unwrap();

    let mut quantiser = Quantiser::default();

    for (_x, _y, pixel) in img.pixels() {
        // TODO - find out if I need to convert alpha channel
        let colour = pixel_to_colour(pixel);

        quantiser.add_colour(&colour);
    }

    let colour_count = led_length / 2;
    let palette = quantiser.make_palette(led_length * 2);

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

    let mut line: Vec<[u8; 4]> = Vec::new();

    for point in (0..colour_count).map(|n| n as f64) {
        let gradient_point = gradient.at(point).to_rgba8();
        line.push(gradient_point);
    }

    line.extend(line.clone().into_iter().rev());

    line
}

fn main() {
    const PIN: i32 = 10;
    const LED_LENGTH: usize = 300;

    let artwork_path = "samples/3CD66192B3012027F485E50EAEF5883EC9894B9FF8E3C89C61AA71286764B553_sk_12_cid_1.jpeg";

    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(PIN) // GPIO 10 = SPI0 MOSI
                .count(LED_LENGTH as i32) // Number of LEDs
                .strip_type(StripType::Ws2812)
                .brightness(10) // default: 255
                .build(),
        )
        .build()
        .unwrap();

    let data: Vec<[u8; 4]> = generate_pixel_vector(artwork_path, LED_LENGTH);

    let leds = controller.leds_mut(0);

    for (i, led) in leds.iter_mut().enumerate() {
        let colour = data.get(i).unwrap();
        dbg!(colour);
        *led = *colour;
    }

    controller.render().unwrap();

    // let mut img: RgbImage = ImageBuffer::new(LED_LENGTH as u32, 1);

    // for (index, colour) in line.iter().enumerate() {
    //     img.put_pixel(
    //         index as u32,
    //         0,
    //         Rgb([colour.red as u8, colour.green as u8, colour.blue as u8]),
    //     );
    // }

    // img.save(format!("output/{file};")).unwrap();
}
