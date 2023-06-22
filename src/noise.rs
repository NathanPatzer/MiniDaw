#![allow(non_snake_case)]
#![allow(dead_code)]
use rand::Rng;
use rand::thread_rng;
#[derive(Clone)]
pub struct Noise
{
    amplitude: f32,
    sample_rate: u32
}

impl Noise
{
    pub fn new(A: f32) -> Noise
    {
        Noise{amplitude: A, sample_rate: 44100}
    }

    pub fn gen(&self,_frequency: f32, duration: f32) -> Vec<f32>
    {
        let num_samples = (self.sample_rate as f32 * duration) as usize;
        let mut samples: Vec<f32> = vec![0.0;num_samples];
        let mut rng = thread_rng();
        for i in 0 ..num_samples{
            samples[i as usize] = rng.gen::<f32>() * self.amplitude;
        }
        return samples;
    }
}