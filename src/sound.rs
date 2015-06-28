use std::path;
use std::vec::Vec;

extern crate hound;
extern crate rand;

pub struct MonoSound {
    pub data: Vec<Vec<i16>>
}

impl MonoSound {
    pub fn new() -> MonoSound {
        MonoSound {
            data: Vec::new()
        }
    }

    pub fn read(file: &path::Path, sample_window: usize) -> MonoSound {
        let mut reader = hound::WavReader::open(file).unwrap();
        {
            let spec = reader.spec();
            assert!(spec.channels == 1);
            assert!(spec.bits_per_sample == 16);
        }
        reader.samples::<i16>().fold(MonoSound::new(), |mut sound, sample| {
            if sound.data.is_empty() || sound.data.last().unwrap().len() == sample_window {
                sound.data.push(vec![sample.unwrap()]);
            } else {
                sound.data.last_mut().unwrap().push(sample.unwrap());
            }
            sound
        })
    }

    pub fn scale(&self) -> i16 {
        let mut min = 0i16;
        let mut max = 0i16;
        for v in self.data.iter() {
            for h in v.iter() {
                if *h > max {
                    max = *h;
                }
                if *h < min {
                    min = *h
                }
            }
        }
        return max - min
    }

    pub fn add_noise(&mut self) {
        let mut rng = rand::thread_rng();
        let scale = self.scale();
        let rscale = scale / 10;
        for v in self.data.iter_mut() {
            for h in v.iter_mut() {
                *h = *h + rand::sample(&mut rng, -rscale..rscale, 1).first().unwrap();
            }
        }
    }
}
