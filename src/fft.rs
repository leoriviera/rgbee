use std::cmp;
use std::fs;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, Error, ValueOr};
use rustfft::{num_complex::Complex, Fft, FftPlanner};

// use spectrum_analyzer::scaling::divide_by_N_sqrt;
// use spectrum_analyzer::windows::hann_window;
// use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};

pub fn fft(b: &[i32], led_length: usize) -> Vec<f32> {
    // if buf.len() == 0 { return 0f64; }
    // let mut sum = 0f64;
    // for &x in buf {
    //     sum += (x as f64) * (x as f64);
    // }
    // let r = (sum / (buf.len() as f64).sqrt();
    // // Convert value to decibels
    // 20.0 * (r / (i16::MAX as f64)).log10()

    dbg!("Starting light intensity calulcations.");

    let num_samples = b.iter().len();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num_samples);

    let mut buffer = b
        .iter()
        .map(|n| Complex {
            re: *n as f32,
            im: 0.0f32,
        })
        .collect::<Vec<_>>();

    fft.process(&mut buffer);

    let amplitudes: Vec<f32> = buffer
        .iter()
        .skip(1)
        .take(num_samples / 2)
        .map(|f| f32::sqrt(f.re.powi(2) + f.im.powi(2)))
        .collect::<Vec<_>>();

    dbg!(amplitudes.iter().len());

    // Multiply outliers by 1.5, multliply everything in range by 0.5
    // Write function
    // fn get_intensity_multiplier(amplitudes: &[f32]) -> [f32]

    // let bin_size = 44100 / amplitudes.iter().len();

    // let start_bin = 60 / bin_size;
    // let end_bin = 6_000 / bin_size;

    // let chunk_count = (end_bin - start_bin) / (led_length / 2);

    // // TODO - Double check this is correct for extracting the frequences
    // let chunked_amplitudes = amplitudes
    //     .clone()
    //     .into_iter()
    //     .skip(start_bin)
    //     .take(end_bin - start_bin)
    //     .collect::<Vec<_>>()
    //     .chunks(chunk_count)
    //     .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
    //     .collect::<Vec<_>>();

    // fs::write(
    //     "/tmp/foo",

    // )
    // .expect("Unable to write file");

    // let amplitudes = buffer
    //     .iter()
    //     .take(num_samples / 2)
    //     .map(|f| f.norm())
    //     .collect::<Vec<_>>();

    // let db = amplitudes
    //     .iter()
    //     .map(|a| 10.0 * a.log10())
    //     .collect::<Vec<_>>();

    // // TODO - Double check if these are the correct decibel calculations
    // let chunked_db = db
    //     .chunks(led_length / 2)
    //     .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
    //     .collect::<Vec<_>>();

    // // TODO - account for the fact human hearing is logarithmic?

    // let x_min = chunked_amplitudes
    //     .iter()
    //     .min_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();

    // let x_max = chunked_amplitudes
    //     .iter()
    //     .max_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();

    // let mean = chunked_amplitudes.iter().sum::<f32>() / chunk_count as f32;

    // dbg!(x_min, x_max, mean);

    // let normalise_floor = 0.01;
    // let normalise_ceiling = 1.00;

    // let normalised = chunked_amplitudes
    //     .iter()
    //     .map(|x| {
    //         normalise_floor
    //             + ((x - x_min) * (normalise_ceiling - normalise_floor) / (x_max - x_min))
    //     })
    //     .collect::<Vec<_>>();

    // let normalised = chunked_amplitudes
    //     .iter()
    //     .map(|x| x.abs() / 250.0)
    //     .collect::<Vec<_>>();

    // dbg!(&normalised);

    // let intensity = db.iter().map(|i| i / 240.0f32).collect::<Vec<_>>();

    // // dbg!(&normalised_intensity);

    // // dbg!("Light intensity calculated.");

    // intensity

    Vec::new()

    // dbg!(normalised.iter().len());

    // normalised
}
