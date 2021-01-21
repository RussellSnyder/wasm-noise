use std::sync::Mutex;
use std::sync::Arc;
use getrandom::getrandom;
use web_sys::console;

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
        (self.sample_clock * 440.0 * 2.0 * std::f32::consts::PI / self.sample_rate).sin()
    }

    pub fn white_noise(&mut self) -> f32 {
        self.sample_clock = (self.sample_clock + 1.0) % self.sample_rate;
        // u8 is between 0 and 255
        // buffer size can't be set dynamically....
        let mut freq_buff = [0u8; 50];
        getrandom(&mut freq_buff).unwrap();

        let n: f32 = freq_buff
            .iter()
            .map(|n| (n / 255) as f32)
            .sum();

        // n will be between 0 and 12750
        self.sample_clock * n
    }
}

pub type AudioProcessorHandle = Arc<Mutex<AudioProcessor>>;
