#![allow(non_snake_case)]
#![allow(dead_code)]
use std::f32::consts::PI;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator};

use crate::waveforms::Generate;
#[derive(Clone)]
pub struct Saw
{
    sample_rate: u32,
    amplitude: f32
}

impl Saw
{
    pub fn new(A: f32) -> Saw
    {
        Saw { sample_rate: 44100 ,amplitude: A}
    }
}

impl Generate for Saw
{
    fn gen(&self,frequency: f32, duration: f32) -> Vec<f32> {
        let num_samples = (self.sample_rate as f32 * duration) as usize;
        let nyquist_frequency = self.sample_rate as f32 / 2.0;
        let mut samples: Vec<f32> = vec![0.0;num_samples];

        samples.par_iter_mut().enumerate().for_each(|(i,sample)|
        {
            let mut s: f32 = 0.0;
            let t = i as f32 / self.sample_rate as f32;
            let mut h = 1;
            while h as f32 * frequency < nyquist_frequency
            {
                let harmonic_frequency = h as f32 * frequency;
                let harmonic_amplitude = self.amplitude / h as f32;
                s = s + (harmonic_amplitude * (t * harmonic_frequency * 2.0 * PI).sin());
                h+=1;
            }
            *sample = s;
        });
        return samples;
    }
}