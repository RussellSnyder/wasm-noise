mod audio_processor;

use std::sync::Mutex;
use std::sync::Arc;
use crate::audio_processor::AudioProcessorHandle;
use crate::audio_processor::AudioProcessor;
use wasm_bindgen::prelude::*;
use web_sys::console;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;

#[wasm_bindgen]
pub struct Handle(Stream);

#[derive(Debug, Copy, Clone)]
enum NoiseType {
    White,
    Pink,
}

fn convert_string_to_noise_type(noise_type: String) -> NoiseType {
    match noise_type.as_str() {
        "White" => NoiseType::White,
        "Pink" => NoiseType::Pink,
        _ => panic!("Unkown NoiseType {}", noise_type),
    }
}

fn get_config() -> (cpal::Device, cpal::SupportedStreamConfig) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config().unwrap();
    (device, config)
}

#[wasm_bindgen]
pub fn play_noise(noise_type: String) -> Handle {
    let noise_type = convert_string_to_noise_type(noise_type);

    // log!("playing noise type: {:?}", noise_type);

    let (device, config) = get_config();

    Handle(match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), noise_type),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), noise_type),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), noise_type),
    })
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, noise_type: NoiseType) -> Stream
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0;
    let channels = config.channels as usize;

    let audio_processor = Arc::new(Mutex::new(AudioProcessor::new(sample_rate)));
    let audio_processor2 = audio_processor.clone();

    let err_fn = |err| console::error_1(&format!("an error occurred on stream: {}", err).into());

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _| write_data(data, channels, &audio_processor2, noise_type),
            err_fn,
        )
        .unwrap();
    stream.play().unwrap();
    stream
}

fn write_data<T>(output: &mut [T], channels: usize, audio_processor: &AudioProcessorHandle, noise_type: NoiseType)
where
    T: cpal::Sample,
{
    if let Ok(mut audio_processor) = audio_processor.try_lock() {
        for frame in output.chunks_mut(channels) {
            let value: T = match noise_type {
                NoiseType::White => cpal::Sample::from::<f32>(&audio_processor.white_noise()),
                NoiseType::Pink => cpal::Sample::from::<f32>(&audio_processor.pink_noise()),
            };
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }    
    } else {
        for frame in output.chunks_mut(channels) {
            for sample in frame.iter_mut() {
                let value: T = cpal::Sample::from::<f32>(&(0f32));
                *sample = value;
            }
        }        
    }
}
