use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr, Error};
use rustfft::{Fft, FftPlanner, num_complex::Complex};

pub fn fft(b: &[i16], led_length: usize) -> Vec<f64> {
    // if buf.len() == 0 { return 0f64; }
    // let mut sum = 0f64;
    // for &x in buf {
    //     sum += (x as f64) * (x as f64);
    // }
    // let r = (sum / (buf.len() as f64)).sqrt();
    // // Convert value to decibels
    // 20.0 * (r / (i16::MAX as f64)).log10()

    let num_samples = 8192;

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num_samples);

    let mut buffer = b.iter()
        .map(|n| Complex { re: *n as f64, im: 0.0f64 })
        .collect::<Vec<_>>();

    fft.process(&mut buffer);

    // TODO - check if the values in this vector need to be normalised
    buffer.iter()
        .take(num_samples / 2)
        .map(|f| f.norm())
        .collect::<Vec<_>>()
        .chunks(led_length)
        .map(|chunk| chunk.iter().sum::<f64>() / chunk.len() as f64)
        .collect::<Vec<_>>()
}

pub fn start_capture(device: &str) -> Result<PCM, Error> {
    let pcm = PCM::new(device, Direction::Capture, false)?;

    // TODO - figure out why this works
    {
        // For this example, we assume 44100Hz, one channel, 16 bit audio.
        // TODO - check assumptions
        let hwp = HwParams::any(&pcm)?;
        hwp.set_channels(1)?;
        hwp.set_rate(44100, ValueOr::Nearest)?;
        hwp.set_format(Format::s16())?;
        hwp.set_access(Access::RWInterleaved)?;
        pcm.hw_params(&hwp)?;
    }

    pcm.start()?;
    Ok(pcm)
}
