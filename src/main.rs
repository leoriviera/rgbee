// Adapted from alsa-rs github examples
use alsa::pcm::*;
use alsa::{Direction, Error, ValueOr};
use hound::{SampleFormat, WavWriter};

fn start_capture(device: &str) -> Result<PCM, Error> {
    let pcm = PCM::new(device, Direction::Capture, false)?;
    {
        // For this example, we assume 44100Hz, one channel, 16 bit audio.
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

fn read(pcm: &PCM) {
    let io = pcm.io_i16().unwrap();

    let num_samples = 44100 * 10;
    let mut buffer = vec![0i16; num_samples];
    for i in 0..num_samples {
        let sample = io.readi(&mut buffer[i..i + 1]).unwrap();
        if sample == 0 {
            break;
        }
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create("output3.wav", spec).unwrap();
    for sample in buffer {
        writer.write_sample(sample).unwrap();
    }
}

fn main() {

    let capture = start_capture("default").unwrap();
    read(&capture);
}
