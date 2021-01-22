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

    // between 0 and 765
    let max = 3 * 255;
    let sum: i32 = buff.iter().map(|n| (*n as i32)).sum();

    // betwen 0.0 and 1.0
    (sum as f32) / (max as f32)
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

    fn sine(&mut self, freq: f32) -> f32 {
        ((self.sample_clock as f32) * freq * 2.0 * std::f32::consts::PI / (self.sample_rate as f32)).sin()
    }

    pub fn white_noise(&mut self) -> f32 {
        // -1.0 and 1.0
        random_f32() * 2.0 - 1.0
    }

    // https://www.musicdsp.org/en/latest/Synthesis/244-direct-pink-noise-synthesis-with-auto-correlated-generator.html?highlight=pink%20noise
    pub fn pink_noise(&mut self) -> f32 {
        let a_array = vec![14055.0, 12759.0, 10733.0, 12273.0, 15716.0];
        let p_array = vec![22347.0, 27917.0, 29523.0, 29942.0, 30007.0 ];
        let mut contrib = vec![0.0,0.0,0.0,0.0,0.0];

        let randu = random_f32() * 32767.0;
        let randv = random_f32() * 32767.0 * 2.0 - 32767.0;

        let mut accum = 0.0;

        for (i, p) in p_array.iter().enumerate() {
            if randu < *p {
                accum -= contrib[i];
                contrib[i] = randv * (a_array[i] as f32);
                accum += contrib[i];
                break;
            }
        }

        let pink_noise = (accum as i64) >> 16;
        pink_noise as f32
    }    

    // pub fn pink_noise(&mut self) -> f32 {
    //     let exponential_decay = 200.0;

    //     self.increment_sample_clock();
    //     let random_f32 = random_f32();
    //     let random_freq = random_f32 * (self.sample_rate as f32) / 2.0;
    //     let amp = 1.0 / (random_freq / exponential_decay);
    //     self.sine(random_freq) * amp
    // }    
}

pub type AudioProcessorHandle = Arc<Mutex<AudioProcessor>>;
