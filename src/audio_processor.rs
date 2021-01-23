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


#[derive(Debug, Clone)]
struct NoiseElement {
    occurance: usize, // 1 is every sample, max is sample rate
    random_vec: Vec<f32>,
    random_vec_index: usize,
    sample_rate: usize,
}

impl NoiseElement {
    fn new(occurance: usize, min: f32, max: f32, sample_rate: usize) -> NoiseElement {
        NoiseElement {
            occurance,
            random_vec: create_random_vec_f32(min, max, sample_rate),
            random_vec_index: 0,
            sample_rate,
        }
    }
    fn next(&mut self, sample_clock: usize) -> f32 {
        if sample_clock % self.occurance == 0 {
            self.random_vec_index = (self.random_vec_index + 1) % self.sample_rate;
            return self.random_vec[self.random_vec_index];
        }
        return 0f32;
    }
}

fn clamp(num: f32) -> f32 {
    if num > 1.0 {
        return 1.0;
    }
    if num < -1.0 {
        return -1.0;
    }
    num
}

// Creates a random f32 vector with values between -1 and 1
fn create_random_vec_f32(min: f32, max: f32, size: usize) -> Vec<f32> {
    if max == min {
        panic!("max and min must be different")
    } else if max < min {
        panic!("max must be greater than min")
    }

    // u8 is between 0 and 255
    // buffer size can't be set dynamically....
    let mut buff: Vec<u8> = Vec::with_capacity(size);
    for _ in 0..size {
        buff.push(0);
    }
    getrandom(&mut buff).unwrap();

    let diff = max - min;
    // convert to floats
    buff.iter()
        .map(|n| (*n as f32) / 255.0 * diff - diff / 2.0)
        // .map(|f| (f - min) / (max - min))
        .collect()
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
    sample_clock: usize,
    sample_rate: usize,
    element_a: NoiseElement,
}

impl AudioProcessor {
    pub fn new(sample_rate: usize) -> AudioProcessor {
        AudioProcessor {
            sample_clock: 0,
            sample_rate,
            element_a: NoiseElement::new(1, -1.0, 1.0, sample_rate),
        }
    }

    fn increment_sample_clock(&mut self) {
        self.sample_clock = (self.sample_clock + 1) % self.sample_rate;
    }

    fn sine(&mut self, freq: f32) -> f32 {
        ((self.sample_clock as f32) * freq * 2.0 * std::f32::consts::PI / (self.sample_rate as f32))
            .sin()
    }

    pub fn white_noise(&mut self) -> f32 {
        // -1.0 and 1.0
        random_f32() * 2.0 - 1.0
    }

    pub fn white_noise_alt(&mut self) -> f32 {
        self.increment_sample_clock();
        // let range = Range::new(0.0, 1.0);
        // let mut noise_element = NoiseElement::new(1, range, self.sample_rate);
        // -1.0 and 1.0
        self.element_a.next(self.sample_clock)
    }

    // https://www.musicdsp.org/en/latest/Synthesis/244-direct-pink-noise-synthesis-with-auto-correlated-generator.html?highlight=pink%20noise
    pub fn pink_noise(&mut self) -> f32 {
        let a_array = vec![14055.0, 12759.0, 10733.0, 12273.0, 15716.0];
        let p_array = vec![22347.0, 27917.0, 29523.0, 29942.0, 30007.0];
        let mut contrib = vec![0.0, 0.0, 0.0, 0.0, 0.0];

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_random_vec_f32_is_correct_size() {
        let mock_size = 400;
        let random_vec = create_random_vec_f32(-1.0, 1.0, mock_size);

        assert_eq!(random_vec.len(), mock_size);
    }

    #[test]
    fn create_random_vec_f32_no_elements_less_than_min() {
        let random_vec = create_random_vec_f32(-1.0, 1.0, 99);

        let less_than_min: Vec<f32> = random_vec
            .clone()
            .into_iter()
            .filter(|n| n < &-1.0)
            .collect();

        assert_eq!(less_than_min.len(), 0);
    }

    #[test]
    fn create_random_vec_f32_no_elements_greater_than_max() {
        let random_vec = create_random_vec_f32(-1.0, 1.0, 99);

        let greater_than_max: Vec<f32> = random_vec
            .clone()
            .into_iter()
            .filter(|n| n > &1.0)
            .collect();

        assert_eq!(greater_than_max.len(), 0);
    }

    #[test]
    fn noise_element_next_returns_f32_in_range() {
        let mock_sample_rate = 100;
        let min = -1.0;
        let max = 1.0;
        let mut noise_element = NoiseElement::new(1, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for _ in 1..mock_sample_rate * 2 {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            assert_ge!(result, min);
            assert_le!(result, max);
        }
    }

    #[test]
    fn noise_element_next_returns_f32_in_range_2() {
        let mock_sample_rate = 100;
        let min = -0.5;
        let max = 0.5;
        let mut noise_element = NoiseElement::new(1, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for _ in 1..mock_sample_rate * 2 {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            assert_ge!(result, min);
            assert_le!(result, max);
        }
    }

    #[test]
    #[should_panic(expected = "max and min must be different")]
    fn noise_element_same_min_max_panics() {
        let mock_sample_rate = 100;
        let min = 1.0;
        let max = 1.0;
        NoiseElement::new(1, min, max, mock_sample_rate);
    }

    #[test]
    #[should_panic(expected = "max must be greater than min")]
    fn noise_element_large_min_panics() {
        let mock_sample_rate = 100;
        let min = 1.0;
        let max = 0.5;
        NoiseElement::new(1, min, max, mock_sample_rate);
    }

    #[test]
    fn noise_element_next_returns_f32_in_range_3() {
        let mock_sample_rate = 100;
        let min = 0.01;
        let max = 1.0;
        let diff = max - min;
        let mut noise_element = NoiseElement::new(1, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for _ in 1..mock_sample_rate * 2 {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            assert_ge!(result, -diff);
            assert_le!(result, diff);
        }
    }

    #[test]
    fn noise_element_next_cycles_through() {
        let mock_sample_rate = 100;
        let mut noise_element = NoiseElement::new(1, -1.0, 1.0, mock_sample_rate);

        let mut sample_clock = 0;
        for _ in 1..mock_sample_rate * 2 {
            sample_clock = sample_clock + 1;
            let _ = noise_element.next(sample_clock);
        };
    }

    #[test]
    fn noise_element_next_occurance_rate_2() {
        let occurance_rate = 2;
        let mock_sample_rate = 100;
        let min = -1.0;
        let max = 1.0;
        let diff = max - min;

        let mut noise_element = NoiseElement::new(occurance_rate, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for i in 0..mock_sample_rate {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            if i % occurance_rate != 0 {
                assert_ge!(result, -diff);
                assert_le!(result, diff);    
            } else {
                assert_eq!(result, 0.0);
            }
        }
    }

    #[test]
    fn noise_element_next_occurance_rate_7() {
        let occurance_rate = 7;

        let mock_sample_rate = 100;
        let min = -1.0;
        let max = 1.0;
        let diff = max - min;

        let mut noise_element = NoiseElement::new(occurance_rate, min, max, mock_sample_rate);

        let mut sample_clock = 0;
        for i in 0..mock_sample_rate {
            sample_clock = sample_clock + 1;

            let result = noise_element.next(sample_clock);
            if i % occurance_rate != 0 {
                assert_ge!(result, -diff);
                assert_le!(result, diff);    
            } else {
                assert_eq!(result, 0.0);
            }
        }
    }


    #[test]
    fn clamp_works() {
        let result_high = clamp(1.5);
        assert_eq!(result_high, 1.0);

        let result_low = clamp(-4.5);
        assert_eq!(result_low, -1.0);

        let result_pass = clamp(0.5);
        assert_eq!(result_pass, 0.5);
    }
}
