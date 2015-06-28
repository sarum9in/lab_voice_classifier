use std::path;
use std::vec::Vec;

extern crate hound;

pub struct MonoSound {
    pub data: Vec<Vec<i16>>
}

impl MonoSound {
    pub fn read(file: &path::Path, sample_window: usize) -> MonoSound {
        let mut reader = hound::WavReader::open(file).unwrap();
        let mut sound = MonoSound {
            data: Vec::new()
        };
        {
            let spec = reader.spec();
            assert!(spec.channels == 1);
            assert!(spec.bits_per_sample == 16);
        }
        reader.samples::<i16>().fold(sound, |mut sound, sample| {
            if (sound.data.is_empty() || sound.data.last().unwrap().len() == sample_window) {
                sound.data.push(vec![sample.unwrap()]);
            } else {
                sound.data.last_mut().unwrap().push(sample.unwrap());
            }
            sound
        })
    }
}
