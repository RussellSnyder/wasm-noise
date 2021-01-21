use getrandom::getrandom;
use std::sync::Arc;
use std::sync::Mutex;
use web_sys::console::log;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn random_f32() -> f32 {
    // u8 is between 0 and 255
    // buffer size can't be set dynamically....
    let mut buff = [0u8; 3];
    getrandom(&mut buff).unwrap();

    let sum: f32 = buff.iter().map(|n| ((*n as f32) - 127.0) / 127.0).sum();

    // should be betwen -1.0 and 1.0
    sum / 3.0
}

pub struct AudioProcessor {
    sample_clock: u32,
    sample_rate: u32,
}

impl AudioProcessor {
    pub fn new(sample_rate: u32) -> AudioProcessor {
        AudioProcessor {
            sample_clock: 0u32,
            sample_rate,
        }
    }

    fn increment_sample_clock(&mut self) {
        self.sample_clock = (self.sample_clock + 1) % self.sample_rate;
    }

    pub fn white_noise(&mut self) -> f32 {
        random_f32()
    }

    pub fn pink_noise(&mut self) -> f32 {
        self.increment_sample_clock();

        // between 0 and 1
        let random = random_f32();

        if random < 0.5 {
            random
        } else {
            random * random_f32()
        }        
    }
}

pub type AudioProcessorHandle = Arc<Mutex<AudioProcessor>>;
