use sdl2::{audio::{AudioQueue, AudioSpecDesired}, sys::{SDL_Delay, SDL_PauseAudio, SDL_PauseAudioDevice, SDL_QueueAudio}};
use std::{time::Duration};
use std::thread;

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
    pub length_counter_index: u8,
    pub timer_high: u8,
    pub duty_array: [u8; 8],
    pub duty_index: u8,
    pub length_counter_value: u8,
    pub combined_timer: u16,
    pub current_timer: u16,
    pub current_volume: u8,
    pub current_length_counter: u8,
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
            length_counter_index: 0,
            timer_high: 0,
            duty_array: [0, 1, 0, 0, 0, 0, 0, 0],
            duty_index: 0,
            length_counter_value: 0,
            combined_timer: 0,
            current_timer: 0,
            current_volume: 0,
            current_length_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        // Implement the pulse channel tick logic here
        // This function should be called every CPU cycle to update the pulse channel state
        if(self.current_timer == 0){
            self.current_timer = self.combined_timer;
            self.duty_index = (self.duty_index + 1) % 8;
            if(!self.constant_volume){
                if(self.current_volume > 0){
                    self.current_volume -= 1;
                }else{
                    if(self.envelope_loop){
                        self.current_volume = self.envelope_volume;
                    }else{
                        self.current_volume = 0;
                    }
                }
            }
        } else {
            self.current_timer -= 1;
            if(!self.constant_volume){
                if(self.current_volume > 0){
                    self.current_volume -= 1;
                }
            }
        }

        if(self.current_length_counter > 0 && !self.envelope_loop){
            self.current_length_counter -= 1;
        }else {
            if(!self.envelope_loop){
                self.current_length_counter = self.length_counter_value;
            }
        }

    }

    pub fn set_duty(&mut self, duty: u8) {
        self.duty = duty;
        self.duty_array = match duty {
            0 => [0, 1, 0, 0, 0, 0, 0, 0],
            1 => [0, 1, 1, 0, 0, 0, 0, 0],
            2 => [0, 1, 1, 1, 1, 0, 0, 0],
            3 => [1, 0, 0, 1, 1, 1, 1, 1],
            _ => !panic!("Invalid duty value"), // Default case for invalid duty values
        };
    }

    pub fn set_length_counter(&mut self, value: u8) {
        self.length_counter_index = value;
        self.length_counter_value = match self.length_counter_index {
            0 => 10,
            1 => 254,
            2 => 20,
            3 => 2,
            4 => 40,
            5 => 4,
            6 => 80,
            7 => 6,
            8 => 160,
            9 => 8,
            10 => 60,
            11 => 10,
            12 => 14,
            13 => 12,
            14 => 26,
            15 => 14,
            16 => 12,
            17 => 16,
            18 => 24,
            19 => 18,
            20 => 48,
            21 => 20,
            22 => 96,
            23 => 22,
            24 => 192,
            25 => 24,
            26 => 72,
            27 => 26,
            28 => 16,
            29 => 28,
            30 => 32,
            31 => 30,
            _ => panic!("Invalid length counter value"),
        };
        self.length_counter_value = value;
    }

    pub fn get_sample(&self) -> f32 {
        // Implement the logic to generate the audio sample for the pulse channel
        // This function should return a floating-point value representing the audio sample
        if(self.duty_array[self.duty_index as usize] == 0){
            return 0.0;
        } 
        
        if(self.current_length_counter <= 0){
            return 0.0;
        }

        //eprint!("{}", self.current_volume as f32 / 15.0);
        return self.current_volume as f32 / 15.0 // Normalize volume to [0.0, 1.0]
    }

    pub fn write_0x4000(&mut self, data: u8) {
        self.duty = (data >> 6) & 0b11;
        self.set_duty(self.duty);
        self.envelope_loop = (data & 0b00100000) != 0;
        self.constant_volume = (data & 0b00010000) != 0;
        self.envelope_volume = data & 0b00001111;
    }

    pub fn write_0x4002(&mut self, data: u8) {
        self.timer_low = data;
        self.combined_timer = (self.timer_high as u16) << 8 | (self.timer_low as u16);
        self.current_timer = self.combined_timer;
    }

    pub fn write_0x4003(&mut self, data: u8) {
        self.timer_high = data & 0b00000111;
        self.combined_timer = (self.timer_high as u16) << 8 | (self.timer_low as u16);
        self.set_length_counter((data >> 3) & 0b00011111);
        self.current_timer = self.combined_timer;
        self.duty_index = 0;
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
    pub cpu_cycles: u64,
    pub device: AudioQueue<f32>,
    pub sample_data: Vec<f32>,
    pub sample_index: u16,
}

impl apu {
    pub fn new(sdl_context: sdl2::Sdl) -> Self {
        apu {
            pulse1: Pulse::new(),
            pulse2: Pulse::new(),
            triangle: Triangle::new(),
            noise: Noise::new(),
            //dmc: DMC::new(),
            cpu_cycles: 0,
            device: sdl_context.audio().unwrap().open_queue::<f32, _>(None, &AudioSpecDesired {
                freq: Some(44100),
                channels: Some(1),
                samples: Some(512)
            }).unwrap(),
            sample_data: Vec::with_capacity(512),
            sample_index: 0,
        }
    }

    pub fn startAudio(&mut self) {
        // Implement the audio output logic here
        // This function should be called to start generating audio samples
        eprint!("Starting APU audio output...");
        self.device.resume();
    }

    pub fn tick(&mut self) {
        // Implement the APU tick logic here
        // This function should be called every CPU cycle to update the APU state
        self.cpu_cycles += 1;
        if(self.cpu_cycles % 2 == 0){
            // Update pulse channels
            self.cpu_cycles = 0;
            self.pulse1.tick();
            self.sample_data.push(self.pulse1.get_sample());
            self.sample_index += 1;
            if(self.sample_index >= 512){
                self.device.queue(&self.sample_data);
                self.sample_data.clear();
                self.sample_index = 0;
            }
            // self.pulse2.tick();
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

    #[test]
    fn apu_pulse1_test() {
        use sdl2::{audio::{AudioQueue, AudioSpecDesired}, sys::{SDL_Delay, SDL_PauseAudio, SDL_PauseAudioDevice, SDL_QueueAudio}};
        use std::time::Duration;
        use std::thread;
        use crate::apu::*;

        let mut apu = apu::new(sdl2::init().unwrap());
        apu.pulse1.set_duty(3);
        apu.pulse1.envelope_loop = false;
        apu.pulse1.constant_volume = false;
        apu.pulse1.envelope_volume = 15;
        apu.pulse1.current_timer = 100;
        apu.pulse1.current_length_counter = 10;

        apu.startAudio();

        for i in 0..44100{
            apu.tick();
            if(i % 2 == 0){
                eprint!("{}", apu.pulse1.current_volume);
                //eprint!("Current Length Counter: {}", apu.pulse1.current_length_counter);
                eprint!(" ");
            }
            thread::sleep(Duration::from_micros(22)); // Simulate CPU cycles (approx. 1/44100 seconds)
        }

    }
}
