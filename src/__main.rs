mod colour;
mod fft;
mod node;
mod quantiser;

use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr};
use colorgrad::Color;
use hsl::HSL;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rand::prelude::*;
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
use rustfft::{num_complex::Complex, FftPlanner};

use colour::Colour;
use fft::{fft, start_capture};
use quantiser::Quantiser;

pub fn morph(start: &Vec<[u8; 4]>, finish: &Vec<[u8; 4]>, steps: usize) -> Vec<Vec<[u8; 4]>> {
    let pixel_count = start.len();
    let targets = start.iter().zip(finish.iter());

    let mut morphs = Vec::with_capacity(steps);

    for _ in 0..steps {
        morphs.push(Vec::with_capacity(pixel_count));
    }

    for pixels in targets {
        let gradient = colorgrad::CustomGradient::new()
            .colors(&[
                Color::from_rgba8(pixels.0[0], pixels.0[1], pixels.0[2], pixels.0[3]),
                Color::from_rgba8(pixels.1[0], pixels.1[1], pixels.1[2], pixels.1[3]),
            ])
            .build()
            .unwrap();

        for (row, colour) in gradient.colors(steps).iter().enumerate() {
            let morph = morphs.get_mut(row).unwrap();

            morph.push(colour.to_rgba8());
        }
    }
    morphs
}

fn set_lightness(p: &[u8; 4], l: f64) -> [u8; 4] {
    let hsl = HSL::from_rgb(&[p[0], p[1], p[2]]);

    let (r, g, b) = HSL { l: l.into(), ..hsl }.to_rgb();

    [r, g, b, 0]
}

fn light_loop(
    controller: &mut Controller,
    pcm: &PCM,
    data: &Vec<[u8; 4]>,
) -> Result<(), alsa::Error> {
    const NUM_SAMPLES: usize = 8192;

    let io = pcm.io_i16()?;

    let mut start = data.clone();
    let mut buffer = [0i16; NUM_SAMPLES];

    loop {
        assert_eq!(io.readi(&mut buffer)?, NUM_SAMPLES);

        // TODO - I predict there may be a problem with the above if audio stops playing
        // check if this will help if there is an issue
        // for i in 0..num_samples {
        //     let sample = io.readi(&mut buffer[i..i + 1]).unwrap();
        //     if sample == 0 {
        //         break;
        //     }
        // }

        let values = fft(&buffer, data.len());

        let finish = data
            .iter()
            .zip(values.iter())
            .map(|(pixel, brightness)| set_lightness(pixel, *brightness))
            .collect::<Vec<_>>();

        // TODO - experiment with number of morph steps
        let transition = morph(&start, &finish, 100);

        for colours in transition.iter() {
            let leds = controller.leds_mut(0);

            for (i, pixel) in colours.iter().enumerate() {
                leds[i] = *pixel;
            }

            controller.render().unwrap();
            controller.wait().unwrap();
        }

        start = finish;
    }
}

fn main() {
    const PIN: i32 = 10;
    const LED_LENGTH: usize = 300;

    let artwork_path = "samples/definitely_maybe.jpeg";

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
                .brightness(10) // default: 255
                .build(),
        )
        .build()
        .unwrap();
    let pcm = start_capture("default").unwrap();

    let data: Vec<[u8; 4]> = generate_pixel_vector(&img, LED_LENGTH);

    light_loop(&mut controller, &pcm, &data).unwrap();

    // let mut img = ImageBuffer::new(LED_LENGTH as u32, 100);

    // for (x, row) in transition.iter().enumerate() {
    //     for (y, pixel) in row.iter().enumerate() {
    //     img.put_pixel(
    //         x as u32,
    //         y as u32,
    //         Rgba(*pixel),
    //     );
    //     }
    // }

    // for (i, led) in leds.iter_mut().enumerate() {
    //     let colour = data.get(i).unwrap();
    //     dbg!(colour);
    //     *led = *colour;
    // }

    // controller.render().unwrap()

    // for (index, colour) in data.iter().enumerate() {
    //     img.put_pixel(
    //         index as u32,
    //         0,
    //         Rgba(*colour),
    //     );
    // }

    // img.save(format!("output/output.jpeg")).unwrap();
}
