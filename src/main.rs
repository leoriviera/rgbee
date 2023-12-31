// Adapted from alsa-rs github examples
mod colour;
mod fft;
mod node;
mod pixels;
mod quantiser;

use alsa::pcm::*;
use alsa::{Direction, Error, ValueOr};
use hound::{SampleFormat, WavWriter};
use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};
use std::{thread, time};

use crate::fft::fft;
use crate::pixels::{generate_pixel_vector, morph};

fn start_capture(device: &str) -> Result<PCM, Error> {
    let pcm = PCM::new(device, Direction::Capture, false)?;
    {
        // TODO - figure out why this works
        // For this example, we assume 44100Hz, one channel, 16 bit audio.
        let hwp = HwParams::any(&pcm)?;
        hwp.set_channels(hwp.get_channels_max()?)?;
        hwp.set_rate(hwp.get_rate_max()?, ValueOr::Nearest)?;
        hwp.set_format(Format::s32())?;
        hwp.set_access(Access::RWInterleaved)?;
        pcm.hw_params(&hwp)?;
    }
    pcm.start()?;
    Ok(pcm)
}

fn read(pcm: &PCM, led_length: usize) -> Vec<f32> {
    let io = pcm.io_i32().unwrap();

    let hw_params = pcm.hw_params_current().unwrap();

    let channels = hw_params.get_channels().unwrap();
    let sample_rate = hw_params.get_rate().unwrap();

    dbg!("Starting audio buffer read.");

    // Read every half a second
    let num_samples = ((sample_rate * channels) / 4) as usize;
    let mut buffer = vec![0i32; num_samples];
    // TODO - recover from EPIPE errors/audio underrun or overrun
    assert_eq!(
        (io.readi(&mut buffer).unwrap() * channels as usize),
        num_samples
    );

    // let spec = hound::WavSpec {
    //     channels: channels as u16,
    //     sample_rate,
    //     bits_per_sample: 32,
    //     sample_format: SampleFormat::Int,
    // };

    // let mut writer = WavWriter::create("output3.wav", spec).unwrap();
    // for sample in &buffer {
    //     writer.write_sample(*sample).unwrap();
    // }

    dbg!("Audio buffer read completed.");

    fft(&buffer, led_length)
}

fn main() {
    // for t in &["pcm", "ctl", "rawmidi", "timer", "seq", "hwdep"] {
    //     println!("{} devices:", t);
    //     let i = HintIter::new(None, &CString::new(*t).unwrap()).unwrap();
    //     for a in i { println!("  {:?}", a) }
    // }

    const PIN: i32 = 10;
    const LED_LENGTH: usize = 10;

    let artwork_path = "samples/crash.png";
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

    let colours = generate_pixel_vector(&img, LED_LENGTH);

    let mut start = colours.clone();

    start.extend(start.clone().into_iter().rev());

    // loop {
    //     let capture = start_capture("hw:3,0,1").unwrap();
    //     let light_intensity = read(&capture, LED_LENGTH);

    //     let mut finish = colours
    //         .iter()
    //         .zip(light_intensity.iter())
    //         .map(|(c, i)| c.set_lightness(*i))
    //         .collect::<Vec<_>>();

    //     finish.extend(finish.clone().into_iter().rev());

    //     let transition = morph(&start, &finish, 250);

    //     for step in transition {
    //         let leds = controller.leds_mut(0);

    //         for (i, pixel) in step.iter().enumerate() {
    //             leds[i] = pixel.to_bgra8();
    //         }

    //         // dbg!("Updating LEDs.", count);
    //         controller.render().unwrap();
    //         controller.wait().unwrap();
    //     }

    //     dbg!("Transitions completed.");
    //     start = finish;
    // }

    let light_intensity = [0.1; LED_LENGTH / 2];

    let mut finish = colours
        .iter()
        .zip(light_intensity.iter())
        .map(|(c, i)| c.set_lightness(*i))
        .collect::<Vec<_>>();

    finish.extend(finish.clone().into_iter().rev());

    dbg!(start.iter().len(), finish.iter().len());

    loop {
        let transition = morph(&start, &finish, 1);

        for step in transition {
            let leds = controller.leds_mut(0);

            for (i, pixel) in step.iter().enumerate() {
                leds[i] = pixel.to_bgra8();
            }

            // dbg!("Updating LEDs.", count);
            controller.render().unwrap();
            controller.wait().unwrap();
        }

        // dbg!("Transitions completed.");

        // (start, finish) = (finish, start);

        // thread::sleep(time::Duration::from_millis(1000));
    }
}

// Loopback is card 3
// Read from 3,0,1 with Rust
// Write to 3,1,1 with Shairplay
