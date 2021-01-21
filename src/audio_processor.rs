use web_sys::console::log;
use std::sync::Mutex;
use std::sync::Arc;
use getrandom::getrandom;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct AudioProcessor {
    pub fm: f32,
    sample_clock: f32,
    sample_rate: f32,
}

impl AudioProcessor {
    pub fn new(sample_rate: f32) -> AudioProcessor {
        AudioProcessor {
            fm: 20.0f32,
            sample_clock: 0f32,
            sample_rate,
        }
    }

    pub fn sine(&mut self) -> f32 {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        let sample = (self.sample_clock * 440.0 * 2.0 * std::f32::consts::PI / self.sample_rate).sin();
        // log!("sample: {}", sample);
        sample
    }

    pub fn white_noise(&mut self) -> f32 {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        // u8 is between 0 and 255
        // buffer size can't be set dynamically....
        let mut buff = [0u8; 50];
        getrandom(&mut buff).unwrap();

        let n: f32 = buff
            .iter()
            .map(|n| (n / 255) as f32)
            .sum();

        // n will be between 0 and 12750
        self.sample_clock * n
    }

    pub fn pink_noise(&mut self) -> f32 {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        // u8 is between 0 and 255
        // buffer size can't be set dynamically....
        let mut buff = [0u8; 10];
        getrandom(&mut buff).unwrap();

        let sum: f32 = buff
            .iter()
            .map(|n|
                ((*n as f32) - 127.0) / 127.0
            )
            .sum();

        // should be betwen -1.0 and 1.0
        let normalized = sum / 10.0;

        // log!("norm: {}", normalized);
        // log!("self.sampl_clock: {}", self.sample_clock);

        // let freq = normalized * self.sample_rate;
        // let amp = 12750.0 / n;
        // n will be between 0 and 12750
        // log!("normalized: {}", normalized);
        normalized        
    }
}

pub type AudioProcessorHandle = Arc<Mutex<AudioProcessor>>;
