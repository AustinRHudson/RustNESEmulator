

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
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };
        
        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            Square {
                phase_inc: 620.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();
        
        // Start playback
        device.resume();
        
        // Play for 2 seconds
        std::thread::sleep(Duration::from_millis(2000));
    }
}
