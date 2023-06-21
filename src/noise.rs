#![allow(non_snake_case)]
#![allow(dead_code)]
use rand::Rng;
use rand::thread_rng;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator};
pub struct Noise
{
    length: u32, //seconds
    sample_rate: u32
}

impl Noise
{
    pub fn new(len: u32) -> Noise
    {
        Noise{length: len, sample_rate: 44100}
    }

    pub fn gen(&mut self, Amplitude: f32) -> Vec<f32>
    {
        let mut samples: Vec<f32> = vec![0.0;(self.sample_rate * self.length) as usize];
        let mut rng = thread_rng();
        for i in 0 .. self.length * self.sample_rate{
            samples[i as usize] = rng.gen::<f32>() * Amplitude;
        }
        return samples;
    }
}