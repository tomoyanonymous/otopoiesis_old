use super::audio;
use super::audio::Generator;

use cpal::traits::{DeviceTrait, StreamTrait};

use super::gui;

use core::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

extern crate atomic_float;
use atomic_float::AtomicF32;

extern crate crossbeam;
use crossbeam::atomic::AtomicCell;


#[derive(Copy, Clone)]
pub struct AppState {
    // inputbuf: Arc<Vec::<f32>>,
    pub freq: u32,
    pub dummy: f64,
    // pub guistate: iced::slider::State,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            freq: 440,
            dummy: 100.,
        }
    }
}

pub fn build_ostream(dspstate: Arc<AtomicCell<AppState>>) -> anyhow::Result<cpal::Stream> {
    // let (sender, receiver) = ::std::sync::mpsc::channel::<AppState>();
    // let mut freq_tmp = 440.0;
    let state_c = dspstate.clone();
    let (_host, device, supported_config) = audio::host_odevice_setup()?;
    let config: &cpal::StreamConfig = &supported_config.into();
    let sample_rate = config.sample_rate.0 as f32;
    let sample_clock = 0f32;
    let nchannels = config.channels as usize;
    let mut request = audio::SampleRequestOptions {
        sample_rate,
        sample_clock,
        nchannels,
    };
    let err_fn_o = |err| eprintln!("Error building output sound stream: {}", err);
    // let mut s = state.clone();
    let on_sample = move |o: &mut audio::SampleRequestOptions| -> f32 {
        let f = state_c.as_ref().load().freq as f32;
        o.tick();
        o.tone(f) * 0.1 + o.tone(f * 2.0) * 0.1
    };
    let ostream = device.build_output_stream(
        config,
        move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in output.chunks_mut(request.nchannels) {
                //for now
                let res = on_sample(&mut request);
                let value: f32 = cpal::Sample::from::<f32>(&res);
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn_o,
    )?;
    Ok(ostream)
}

#[derive(Default)]
struct GuiAppState {
    audiostream: Option<cpal::Stream>,
    dspstate: Arc<AtomicCell<AppState>>,
    freqslider: iced::slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SliderChanged(f32),
}
extern crate iced;
use iced::{
    button, executor, slider, Align, Application, Button, Clipboard, Column, Command, Container,
    Element, HorizontalAlignment, Length, Row, Settings, Slider, Text, VerticalAlignment,
};
impl iced::Application for GuiAppState {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (GuiAppState, Command<Self::Message>) {
        let s = Arc::new(AtomicCell::new(AppState::default()));
        let sc = s.clone();
        let ostream = build_ostream(sc).expect("failed to open stream!");

        let _ = &ostream.play();

        (
            Self {
                audiostream: Some(ostream),
                dspstate: s,
                freqslider: slider::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Freq test")
    }

    fn update(&mut self, message: Message, _clip: &mut Clipboard) -> Command<Message> {
        match message {
            Message::SliderChanged(f) => {
                let a = AppState {
                    freq: f as u32,
                    dummy: 0.,
                };
                self.dspstate.as_ref().store(a);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Align::Center)
            .push(
                Slider::new(
                    &mut self.freqslider,
                    1.0..=1000.0,
                    self.dspstate.as_ref().load().freq as f32,
                    Message::SliderChanged,
                )
                .step(0.01),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
impl Drop for GuiAppState {
    fn drop(&mut self) {
        if let Some(s) = &self.audiostream {
            drop(s);
            println!("finished audio stream");
        }
    }
}

pub fn run_app() -> iced::Result {
    GuiAppState::run(Settings::default())
}
