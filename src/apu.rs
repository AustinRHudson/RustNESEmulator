use sdl2::{audio::{AudioQueue, AudioSpecDesired}, sys::{False, SDL_Delay, SDL_PauseAudio, SDL_PauseAudioDevice, SDL_QueueAudio}};
use std::{eprint, mem::transmute, time::Duration};
use std::thread;

pub struct Pulse {
    pub duty: u8,
    pub envelope_loop: bool,
    pub constant_volume: bool,
    pub envelope_volume: u8, //acts as the volume if constant_volume is set to true, otherwise it acts as the period for the envelope generator when constant_volume is set to false
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
    pub sweep_divider_period: u8,
    pub pulse_number: u8,
    pub current_sweep_divider_period: u8,
    pub sweep_muted: bool,
    pub envelope_period: u8,
    pub envelope_divider: u8,
    pub envelope_start: bool,
}

impl Pulse {
    pub fn new(num: u8) -> Self {
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
            sweep_divider_period: 0,
            pulse_number: num,
            current_sweep_divider_period: 0,
            sweep_muted: false,
            envelope_period: 0,
            envelope_divider: 0,
            envelope_start: false
        }
    }

    pub fn tick(&mut self) {
        // Implement the pulse channel tick logic here
        // This function should be called every CPU cycle to update the pulse channel state
        //eprint!("Current Volume {}\n", self.current_volume);
        //eprint!("Current Timer: {}, Combined Timer: {}, Duty Index: {}, Current Volume: {}, Current Length Counter: {}\n", self.current_timer, self.combined_timer, self.duty_index, self.current_volume, self.current_length_counter);
        if(self.current_timer == 0){
            self.current_timer = self.combined_timer;
            self.duty_index = (self.duty_index + 1) % 8;
        } else {
            self.current_timer -= 1;
        }

    }

    pub fn clock_length_counter(&mut self) {
        if self.current_length_counter > 0
            && !self.envelope_loop
        {
            self.current_length_counter -= 1;
        }
    }

    pub fn clock_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.current_volume = 15;
            self.envelope_divider = self.envelope_period;
            return;
        }
        if self.envelope_divider == 0 {
            self.envelope_divider = self.envelope_period;

            if self.current_volume > 0 {
                self.current_volume -= 1;
            } else if self.envelope_loop {
                self.current_volume = 15;
            }
        } else {
            self.envelope_divider -= 1;
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
        self.current_length_counter = self.length_counter_value;
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

        if(self.sweep_muted){
            return 0.0;
        }

        //eprint!("{}", self.current_volume as f32 / 15.0);
        return self.current_volume as f32 // Normalize volume to [0.0, 1.0]
    }

    pub fn clock_sweep(&mut self) {
        // Implement the sweep unit logic here
        // This function should be called every half frame to update the sweep unit state
        if(self.sweep_enabled && self.shift_count > 0){
            let change_amount = self.combined_timer >> self.shift_count;
            if(self.current_sweep_divider_period == 0){
                self.current_sweep_divider_period = self.sweep_divider_period;
                if(self.negate_flag){
                        if(self.pulse_number == 1){
                            let target = self.combined_timer.wrapping_sub(change_amount).wrapping_sub(1);
                            if(target > 0x7FF){
                                self.sweep_muted = true;
                            }else{
                                self.sweep_muted = false;
                                self.combined_timer = target;
                            }
                        }else{
                            let target = self.combined_timer.wrapping_sub(change_amount);
                            if(target > 0x7FF){
                                self.sweep_muted = true;
                            }else{
                                self.sweep_muted = false;
                                self.combined_timer = target;
                            }
                        }
                } else {
                    let target = change_amount + self.combined_timer;
                    if(target > 0x7FF){
                        self.sweep_muted = true;
                    }else{
                        self.sweep_muted = false;
                        self.combined_timer += change_amount;
                    }
                }
            } else {
                self.current_sweep_divider_period -= 1;

            }
        }
    }

    pub fn write_0x4000(&mut self, data: u8) {
        //eprint!("writing to 0x4000: {:08b}\n", data);
        self.duty = (data >> 6) & 0b11;
        self.set_duty(self.duty);
        self.envelope_loop = (data & 0b00100000) != 0;
        self.constant_volume = (data & 0b00010000) != 0;
        self.envelope_volume = data & 0b00001111;
        self.divider_period = data & 0b00001111;
        self.envelope_period = data & 0x0F;
        self.envelope_divider = self.envelope_period;
        if(self.constant_volume){
            self.current_volume = self.envelope_volume;
        }
    }

    pub fn write_0x4001(&mut self, data: u8) {
        //eprint!("writing to 0x4001: {:08b}\n", data);
        self.sweep_enabled = (data & 0b10000000) != 0;
        self.sweep_divider_period = (data >> 4) & 0b00000111;
        self.negate_flag = (data & 0b00001000) != 0;
        self.shift_count = data & 0b00000111;
        self.current_sweep_divider_period = (data >> 4) & 0b00000111;
    }

    pub fn write_0x4002(&mut self, data: u8) {
        self.timer_low = data;
        self.combined_timer = (self.timer_high as u16) << 8 | (self.timer_low as u16);
        self.current_timer = self.combined_timer;
    }

    pub fn write_0x4003(&mut self, data: u8) {
        //eprint!("writing to 0x4003: {:08b}\n", data);
        self.timer_high = data & 0b00000111;
        self.combined_timer = (self.timer_high as u16) << 8 | (self.timer_low as u16);
        self.set_length_counter((data >> 3) & 0b00011111);
        self.current_timer = self.combined_timer;
        self.duty_index = 0;
        if(self.constant_volume){
            self.current_volume = self.envelope_volume;
        }else{
            self.current_volume = 15;
        }
        self.envelope_start = true;
        //eprint!("Current Volume: {}\n", self.current_volume);
    }
}

pub struct Triangle {
    pub linear_counter_control: bool,
    pub reload_value: u8,
    pub timer_low: u8,
    pub length_counter_index: u8,
    pub length_counter_value: u8,
    pub current_length_counter: u8,
    pub timer_high: u8,
    pub combined_timer: u16,
    pub reload_flag: bool,
    pub current_timer: u16,
    pub sequence_index: u8,
    pub sequence_array: [u8; 32],
    pub current_linear_counter: u8,
}

impl Triangle {
    pub fn new() -> Self {
        Triangle {
            linear_counter_control: false,
            reload_value: 0,
            timer_low: 0,
            length_counter_index: 0,
            length_counter_value: 0,
            current_length_counter: 0,
            timer_high: 0,
            combined_timer: 0,
            reload_flag: false,
            current_timer: 0,
            sequence_index: 0,
            sequence_array: [
            15, 14, 13, 12, 11, 10, 9, 8,
            7,  6,  5,  4,  3,  2, 1, 0,
            0,  1, 2,  3,  4,  5, 6, 7,
            8,  9, 10, 11, 12, 13, 14, 15],
            current_linear_counter: 0,
        }
    }

    pub fn tick(&mut self){
        if(self.current_timer == 0){
            self.current_timer = self.combined_timer;
            self.sequence_index = (self.sequence_index + 1) % 32;
        }else{
            self.current_timer -= 1;
        }
    }

    pub fn clock_linear_counter(&mut self){
        if (self.reload_flag) {
            self.current_linear_counter = self.reload_value;
        } else if (self.current_linear_counter > 0) {
            self.current_linear_counter -= 1;
        }

        if !self.linear_counter_control {
            self.reload_flag = false;
        }
    }

    pub fn clock_length_counter(&mut self){
        if (!self.linear_counter_control && self.current_length_counter > 0)
        {
            self.current_length_counter -= 1;
        }
    }

    pub fn get_sample(&mut self) -> f32{
        if(self.current_length_counter == 0 || self.current_linear_counter == 0){
            return 0.0;
        }
        return self.sequence_array[self.sequence_index as usize] as f32;
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
        self.current_length_counter = self.length_counter_value;
    }

    pub fn write_0x4008(&mut self, data: u8) {
        self.linear_counter_control = (data & 0b1000_0000) != 0;

        self.reload_value = data & 0b0111_1111;
    }

    pub fn write_0x400A(&mut self, data: u8){
        self.timer_low = data;
    }

    pub fn write_0x400B(&mut self, data: u8){
        self.set_length_counter(data >> 3 & 0b00011111);
        self.timer_high = data & 0b0000_0111;
        self.combined_timer = ((self.timer_high as u16) << 8) | self.timer_low as u16;
        self.reload_flag = true;
    }

}

pub struct Noise {
    pub envelope_loop: bool,
    pub constant_volume: bool,
    pub envelope_volume: u8,
    pub noise_mode: bool,
    pub envelope_period: u8,
    pub length_counter: u8,
    pub current_length_counter:u8,
    pub shift_register: u16,
    pub length_counter_value: u8,
    pub shift_timer: u16,
    pub current_shift_timer: u16,
    pub current_volume: u8,
    pub envelope_start: bool,
    pub envelope_divider: u8
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            envelope_loop: false,
            constant_volume: false,
            envelope_volume: 0,
            noise_mode: false,
            envelope_period: 0,
            length_counter: 0,
            current_length_counter: 0,
            shift_register: 1,
            length_counter_value: 0,
            shift_timer: 0,
            current_shift_timer: 0,
            current_volume: 0,
            envelope_start: false,
            envelope_divider: 0,
        }
    }

    pub fn clock_shift_register(&mut self){
        self.current_shift_timer += 2;
        if(self.current_shift_timer >= self.shift_timer){
            self.current_shift_timer = 0;
            let mut feedback: u16;
            if(self.noise_mode){
                feedback = (self.shift_register & 0b0000_0000_0100_0000)^(self.shift_register & 0b0000_0000_0000_0001);
            }else{
                feedback = (self.shift_register & 0b0000_0000_0000_0010)^(self.shift_register & 0b0000_0000_0000_0001);
            }
            self.shift_register >>= 1;
            if(feedback > 0){
                self.shift_register = self.shift_register | 0b0100_0000_0000_0000;
            }else{
                self.shift_register = self.shift_register & 0b1011_1111_1111_1111;
            }
        }
    }

    pub fn clock_length_counter(&mut self) {
        if self.current_length_counter > 0 && !self.envelope_loop
        {
            self.current_length_counter -= 1;
        }
    }

    pub fn clock_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.current_volume = 15;
            self.envelope_divider = self.envelope_period;
            return;
        }

        if self.envelope_divider == 0 {
            self.envelope_divider = self.envelope_period;

            if self.current_volume > 0 {
                self.current_volume -= 1;
            } else if self.envelope_loop {
                self.current_volume = 15;
            }
        } else {
            self.envelope_divider -= 1;
        }
    }

    pub fn get_sample(&mut self) -> f32{
        if(self.current_length_counter == 0 || self.shift_register & 0b0000_0000_0000_0001 == 1){
            return 0.0
        }

        if(self.constant_volume){
            return self.envelope_volume as f32;
        }

        return self.current_volume as f32;
    }

    pub fn write_0x400C(&mut self, data: u8){
        self.envelope_loop = data & 0b0010_0000 != 0;
        self.constant_volume = data & 0b0001_0000 != 0;
        self.envelope_volume = data & 0b0000_1111;
        if(self.constant_volume){
            self.current_volume = self.envelope_volume;
        }
    }

    pub fn write_0x400E(&mut self, data: u8){
        self.noise_mode = data & 0b1000_0000 != 0;

        let period = data & 0b0000_1111;

        self.set_shift_timer(period);
    }

    pub fn write_0x400F(&mut self, data: u8){
        self.set_length_counter((data >> 3)& 0b0001_1111);
        self.envelope_start = true;
    }

    pub fn set_length_counter(&mut self, value: u8) {
        self.length_counter_value = match value {
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
        self.current_length_counter = self.length_counter_value;
    }

    pub fn set_shift_timer(&mut self, data:u8){
        self.shift_timer = match data {
        0  => 4,
        1  => 8,
        2  => 16,
        3  => 32,
        4  => 64,
        5  => 96,
        6  => 128,
        7  => 160,
        8  => 202,
        9  => 254,
        10 => 380,
        11 => 508,
        12 => 762,
        13 => 1016,
        14 => 2034,
        15 => 4068,
        _ => unreachable!(),
    };
    self.current_shift_timer = 0;
    }
}

pub struct dmc {
    pub irq_flag: bool,
    pub loop_flag: bool,
    pub rate_index: u8,
    pub load_counter: u8,
    pub sample_address: u8,
    pub sample_length: u8,

}

impl dmc {
    pub fn new() -> Self{
        dmc {
            irq_flag: false,
            loop_flag: false,
            rate_index: 0,
            load_counter: 0,
            sample_address: 0,
            sample_length: 0,
        }
    }

    pub fn write_0x4010(&mut self, data:u8){
        if(data & 0b1000_0000 > 0){
            self.irq_flag = true;
        }else{
            self.irq_flag = false;
        }

        if(data & 0b0100_0000 > 0){
            self.loop_flag = true;
        }else{
            self.loop_flag = false;
        }

        self.rate_index = data & 0b0000_1111;
    }

    pub fn write_0x4011(&mut self, data:u8){
        self.load_counter = data & 0b0111_1111;
    }

    pub fn write_0x4012(&mut self, data:u8){
        self.sample_address = data;
    }

    pub fn write_0x4013(&mut self, data:u8){
        self.sample_length = data;
    }
}

pub struct apu {
    pub pulse1: Pulse,
    pub pulse2: Pulse,
    pub triangle: Triangle,
    pub noise: Noise,
    pub dmc: dmc,
    pub cpu_cycles: u64,
    pub device: AudioQueue<f32>,
    pub sample_data: Vec<f32>,
    pub sample_index: u16,
    pub status_register: u8,
    pub sample_timer: f64,
}

impl apu {
    pub fn new(device: AudioQueue<f32>) -> Self {
        apu {
            pulse1: Pulse::new(1),
            pulse2: Pulse::new(2),
            triangle: Triangle::new(),
            noise: Noise::new(),
            dmc: dmc::new(),
            cpu_cycles: 0,
            device: device,
            sample_data: Vec::with_capacity(1024),
            sample_index: 0,
            status_register: 0,
            sample_timer: 0.0,
        }
    }

    pub fn tick(&mut self) {
        // Implement the APU tick logic here
        // This function should be called every CPU cycle to update the APU state
        self.cpu_cycles += 1;
        if(self.cpu_cycles % 2 == 0){
            // Update pulse channels
            self.pulse1.tick();
            self.pulse2.tick();
            self.noise.clock_shift_register();
        }

        self.triangle.tick();

        //self.sample_timer += 100_100.0 / 1_789_773.0;

        if (self.cpu_cycles % 20 == 0){
            // Update triangle channel
            //self.sample_timer = 0.0;
            let mut volume: f32 = 0.0;
            if(self.status_register & 0b00000001 > 0){
                volume += self.pulse1.get_sample();
            }
            if(self.status_register & 0b00000010 > 0){
                volume += self.pulse2.get_sample();
            }
            if(volume > 0.0){
                volume = 95.88 / ((8128.0 / volume) + 100.0);
            }
            let mut triangle_volume: f32;
            if(self.status_register & 0b0000_0100 > 0){
               triangle_volume = self.triangle.get_sample();
            }else{
                triangle_volume = 0.0;
            }
            let mut noise: f32;
            if(self.status_register & 0b0000_1000 > 0){
                noise = self.noise.get_sample();
            }else{
                noise = 0.0;
            }
            let mut tnd: f32;
            tnd = triangle_volume / 8227.0 + noise/12241.0;
            if(tnd > 0.0){
                tnd = 159.79 / ((1.0 / tnd) + 100.0);
            }
            
            self.sample_data.push(volume + tnd);
            self.sample_index += 1;
            if(self.sample_index >= 1024){
                //let samples_to_queue: Vec<f32> = self.sample_data.drain(..1024).collect();
                self.device.queue(&self.sample_data);
                self.sample_data.clear();
                self.sample_index = 0;
                //eprint!("Queued Audio Samples: {}\n", self.device.size() / std::mem::size_of::<f32>() as u32);
            }
        }

        if(self.cpu_cycles % 3729 == 0){
            //Quarter frame tick
            //Update envelope and linear counter
            self.pulse1.clock_envelope();
            self.pulse2.clock_envelope();
            self.triangle.clock_linear_counter();
            self.noise.clock_envelope();
        }

        if(self.cpu_cycles % 7457 == 0){
            //Half frame tick
            //Update length counter and sweep unit
            self.pulse1.clock_length_counter();
            self.pulse1.clock_sweep();
            self.pulse2.clock_length_counter();
            self.pulse2.clock_sweep();
            self.triangle.clock_length_counter();
            self.noise.clock_length_counter();
            //eprint!("Current cycles: {}\n", self.cpu_cycles);
        }
    }

    pub fn write_status_register(&mut self, data: u8){
        self.status_register = data;
    }
}

mod testingTime {
    use sdl2::sys::SDL_GetQueuedAudioSize;

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
        samples: Some(512)
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

        let sdl_context = sdl2::init().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: Some(512)
        };

        let device: AudioQueue<f32> = audio_subsystem.open_queue::<f32, _>(None, &desired_spec).unwrap();

        device.resume();

        let mut apu = apu::new(device);
        apu.pulse1.set_duty(0);
        apu.pulse1.envelope_loop = true;
        apu.pulse1.constant_volume = false;
        apu.pulse1.envelope_volume = 15;
        apu.pulse1.current_timer = 100;
        apu.pulse1.current_length_counter = 100;

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

     #[test]
    fn apu_pulse1_test_registers() {
        use sdl2::{audio::{AudioQueue, AudioSpecDesired}, sys::{SDL_Delay, SDL_PauseAudio, SDL_PauseAudioDevice, SDL_QueueAudio}};
        use std::time::Duration;
        use std::thread;
        use crate::apu::*;

        let sdl_context = sdl2::init().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: Some(512)
        };

        let device: AudioQueue<f32> = audio_subsystem.open_queue::<f32, _>(None, &desired_spec).unwrap();

        device.resume();

        let mut apu = apu::new(device);

        apu.pulse1.write_0x4000(0b0001_1111);
        apu.pulse1.write_0x4002(100);
        apu.pulse1.write_0x4003(0b0000_0000);

        let mut queued_bytes = apu.device.size();

// Convert bytes to samples (assuming 16-bit mono audio)
        let mut queued_samples = queued_bytes / std::mem::size_of::<f32>() as u32;

        for i in 0..44100{
            apu.tick();
            if(i % 2 == 0){
                queued_bytes = apu.device.size();
                queued_samples = queued_bytes / std::mem::size_of::<f32>() as u32;
                //eprint!("{}", apu.pulse1.current_volume);
                //eprint!("Current Length Counter: {}", apu.pulse1.current_length_counter);
                //eprint!(" ");
                eprint!("{}", queued_samples);
            }
            thread::sleep(Duration::from_micros(22)); // Simulate CPU cycles (approx. 1/44100 seconds)
        }

    }
}
