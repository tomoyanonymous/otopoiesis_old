use super::core;

/* This example expose parameter to pass generator of sample.
Good starting point for integration of cpal into your application.
*/
extern crate anyhow;
// extern crate clap;
extern crate cpal;
pub use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::borrow::BorrowMut;
use std::sync::Arc;

pub fn run() -> anyhow::Result<()> {
    // let mut gen = SampleRequestOptions::default();

    // let samplfn = move || {
    //     gen.tick();
    //     gen.tone(1) * 0.1 + gen.tone(2) * 0.1
    // };
    // let stream = stream_setup_for(sample_next)?;
    // stream.play()?;
    // std::thread::sleep(std::time::Duration::from_millis(3000));
    Ok(())
}
pub trait Generator: Send + 'static {
    fn tone(&self, freq: f32) -> f32;
    fn tick(&mut self);
    fn get_samplerate(&self) -> f32;
    fn get_channels(&self) -> usize;
}

pub struct SampleRequestOptions {
    pub sample_rate: f32,
    pub sample_clock: f32,
    pub nchannels: usize,
}

unsafe impl Send for SampleRequestOptions {}
unsafe impl Sync for SampleRequestOptions {}

impl Default for SampleRequestOptions {
    fn default() -> Self {
        Self {
            sample_rate: 48000.,
            sample_clock: 0.,
            nchannels: 1,
        }
    }
}

impl Generator for SampleRequestOptions {
    fn tone(&self, freq: f32) -> f32 {
        (self.sample_clock * freq * 2.0 * std::f32::consts::PI / self.sample_rate).sin()
    }
    fn tick(&mut self) {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
    }
    fn get_samplerate(&self) -> f32 {
        self.sample_rate
    }
    fn get_channels(&self) -> usize {
        self.nchannels
    }
}

// pub fn stream_setup_for<F>(on_sample: F) -> Result<cpal::Stream, anyhow::Error>
// where
//     F: FnMut(&mut SampleRequestOptions) -> f32 + Send + 'static,
// {
//     let (_host, device, config) = host_odevice_setup()?;

//     match config.sample_format() {
//         cpal::SampleFormat::F32 => stream_make::<f32, _>(&device, &config.into(), on_sample),
//         cpal::SampleFormat::I16 => stream_make::<i16, _>(&device, &config.into(), on_sample),
//         cpal::SampleFormat::U16 => stream_make::<u16, _>(&device, &config.into(), on_sample),
//     }
// }

pub fn host_odevice_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {:?}", config);

    Ok((host, device, config))
}
// pub fn host_idevice_setup(
// ) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
//     let host = cpal::default_host();

//     let device = host
//         .default_input_device()
//         .ok_or_else(|| anyhow::Error::msg("Default input device is not available"))?;
//     println!("Input device : {}", device.name()?);

//     let config = device.default_input_config()?;
//     println!("Default input config : {:?}", config);

//     Ok((host, device, config))
// }
pub fn get_default_host() -> cpal::Host {
    cpal::default_host()
}

pub enum DeviceKind {
    Input,
    Output,
}

pub fn get_default_device(
    host: &cpal::Host,
    devicekind: DeviceKind,
) -> Result<(cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    match devicekind {
        DeviceKind::Input => {
            let device = host
                .default_input_device()
                .ok_or_else(|| anyhow::Error::msg("Default input device is not available"))?;
            println!("Input device : {}", device.name()?);
            let config = device.default_input_config()?;
            println!("Default input config : {:?}", config);
            Ok((device, config))
        }
        DeviceKind::Output => {
            let device = host
                .default_output_device()
                .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
            println!("Output device : {}", device.name()?);

            let config = device.default_output_config()?;
            println!("Default output config : {:?}", config);
            Ok((device, config))
        }
    }
}

// pub fn stream_make<T, F>(
//     device: &cpal::Device,
//     config: &cpal::StreamConfig,
//     on_sample: F,
// ) -> Result<cpal::Stream, anyhow::Error>
// where
//     T: cpal::Sample,
//     F: FnMut(&mut SampleRequestOptions) -> f32 + Send + 'static,
// {
//     let sample_rate = config.sample_rate.0 as f32;
//     let sample_clock = 0f32;
//     let nchannels = config.channels as usize;
//     let mut request = SampleRequestOptions {
//         sample_rate,
//         sample_clock,
//         nchannels,
//     };
//     let err_fn_o = |err| eprintln!("Error building output sound stream: {}", err);
//     // let mut s = state.clone();

//     let stream = device.build_output_stream(
//         config,
//         move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
//             on_window(output, &mut request, on_sample)
//         },
//         err_fn_o,
//     )?;

//     Ok(stream)
// }

// fn on_window_input<T, F>(input: &[T], request: &SampleRequestOptions, on_sample: F)
// where
//     T: cpal::Sample,
//     F: FnMut(&SampleRequestOptions) -> f32 + std::marker::Send + 'static,
// {
//     for frame in input.chunks(request.nchannels) {
//         let value: T = cpal::Sample::from::<f32>(&on_sample(request));
//         for sample in frame.iter_mut() {
//             *sample = value;
//         }
//     }
// }
// fn on_window<T, F>(output: &mut [T], request: &mut SampleRequestOptions, mut on_sample: F)
// where
//     T: cpal::Sample,
//     F: FnMut(&mut SampleRequestOptions) -> f32 + Send + 'static,
// {
//     for frame in output.chunks_mut(request.nchannels) {
//         //for now
//         let res = on_sample(request);
//         let value: T = cpal::Sample::from::<f32>(&res);
//         for sample in frame.iter_mut() {
//             *sample = value;
//         }
//     }
// }
