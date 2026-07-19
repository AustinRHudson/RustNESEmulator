
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
        use sdl2::audio::{AudioCallback, AudioSpecDesired};
        use std::time::Duration;
        
        struct Square {
            phase_inc: f32,
            phase: f32,
            volume: f32
        }
        
        impl AudioCallback for Square {
            type Channel = f32;
        
            fn callback(&mut self, out: &mut [f32]) {
                // Generate a square wave
                for x in out.iter_mut() {
                    *x = if self.phase <= 0.5 {
                        self.volume
                    } else {
                        -self.volume
                    };
                    self.phase = (self.phase + self.phase_inc) % 1.0;
                    println!("{}", self.phase);
                }
            }
        }
        
        let sdl_context = sdl2::init().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();
        
        let desired_spec = AudioSpecDesired {
            freq: Some(6200),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };
        
        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            Square {
                phase_inc: 620.0 / spec.freq as f32,
                phase: 0.1,
                volume: 0.02
            }
        }).unwrap();
        
        // Start playback
        device.resume();
        
        // Play for 2 seconds
        std::thread::sleep(Duration::from_millis(2000));
    }
}
