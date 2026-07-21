use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioQueue};
use std::time::Duration;
pub struct Pulse {
    pub duty: u8,
    pub envelope_loop: bool,
    pub constant_volume: bool,
    pub envelope_volume: u8,
    pub sweep_enabled: bool,
    pub divider_period: u8,
    pub negate_flag: bool,
    pub shift_count: u8,
    pub timer_low: u8,
    pub length_counter: u8,
    pub timer_high: u8,
}

impl Pulse {
    pub fn new() -> Self {
        Pulse {
            duty: 0,
            envelope_loop: false,
            constant_volume: false,
            envelope_volume: 0,
            sweep_enabled: false,
            divider_period: 0,
            negate_flag: false,
            shift_count: 0,
            timer_low: 0,
            length_counter: 0,
            timer_high: 0,
        }
    }
}

pub struct Triangle {
    pub linear_counter_control: bool,
    pub reload_value: u8,
    pub timer_low: u8,
    pub length_counter: u8,
    pub timer_high: u8,
    pub reload_flag: bool,
}

impl Triangle {
    pub fn new() -> Self {
        Triangle {
            linear_counter_control: false,
            reload_value: 0,
            timer_low: 0,
            length_counter: 0,
            timer_high: 0,
            reload_flag: false,
        }
    }
}

pub struct Noise {
    pub envelope_loop: bool,
    pub constant_volume: bool,
    pub envelope_volume: u8,
    pub noise_mode: bool,
    pub noise_period: u8,
    pub length_counter: u8,
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            envelope_loop: false,
            constant_volume: false,
            envelope_volume: 0,
            noise_mode: false,
            noise_period: 0,
            length_counter: 0,
        }
    }
}
pub struct apu {
    pub pulse1: Pulse,
    pub pulse2: Pulse,
    pub triangle: Triangle,
    pub noise: Noise,
    //pub dmc: DMC,
}

impl apu {
    pub fn new() -> Self {
        apu {
            pulse1: Pulse::new(),
            pulse2: Pulse::new(),
            triangle: Triangle::new(),
            noise: Noise::new(),
            //dmc: DMC::new(),
        }
    }
}

mod testingTime {
    #[cfg(test)]

    #[test]
    
    fn main() {
        use sdl2::{audio::{AudioQueue, AudioSpecDesired}, sys::{SDL_Delay, SDL_PauseAudio, SDL_PauseAudioDevice, SDL_QueueAudio}};
        use std::time::Duration;
        use std::thread;

        let sdl_context = sdl2::init().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: Some(1024)
        };

        let device: AudioQueue<f32> = audio_subsystem.open_queue::<f32, _>(None, &desired_spec).unwrap();

        let sampleRate: i32 = desired_spec.freq.unwrap();

        let mut data: Vec<f32> = Vec::with_capacity(1024);

        eprintln!("Sample Rate: {}", sampleRate);

        let duration: f32 = 20.0; // Duration in seconds

        let totalSamples = desired_spec.samples.unwrap() as f32 * duration;

        let mut count = 0;

        while(count < 5){
            for i in 0..= totalSamples as usize {
                let sample = (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / sampleRate as f32).sin();
                data.push(sample);
            }

        device.queue(&data);

        device.resume();

        thread::sleep(Duration::from_millis(2000));

        count += 1;

        data.clear();
        }
        

        thread::sleep(Duration::from_millis(2000));

        
    }
}
