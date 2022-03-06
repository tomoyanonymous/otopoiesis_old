use super::audio;
use cpal::traits::{DeviceTrait};

use super::gui;
use std::sync::Arc;
use std::ops::Deref;
struct AppState{
    inputbuf: Arc<Vec::<f32>> 
}

pub fn build_istream(state:&mut AppState)->Result<cpal::Stream, anyhow::Error> {
    let ibuf = state.inputbuf.clone();
    let host = audio::get_default_host();
    let (idevice, iconfig) = audio::get_default_device(&host, audio::DeviceKind::Input)?;
    let bufsize:Option<usize> =  match iconfig.buffer_size() {
        cpal::SupportedBufferSize::Range{min,max} => Some(*max as usize),
        cpal::SupportedBufferSize::Unknown => None
    };
    ibuf.deref().resize(bufsize.expect("Buffer Size was Unknown!") ,0.0);

    let stream = idevice.build_input_stream(
        &iconfig.into(),
        move |input: &[f32], _: &cpal::InputCallbackInfo | {
            // for frame in input.chunks(config.channels as usize) {
            for frame in input.chunks(1 as usize) {
                ibuf.copy_from_slice(input);
            }
        },
        move |err| eprintln!("Error building input sound stream: {}", err),
    )?;
    Ok(stream)
}

pub fn run_app() {}
