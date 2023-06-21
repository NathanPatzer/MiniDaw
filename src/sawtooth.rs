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
        let attackTime: f32 = 0.05;
        let decayTime: f32 = 0.05;
        let decaySamples = (self.sample_rate as f32 * decayTime) as usize;
        let attackSamples = (self.sample_rate as f32 * attackTime) as usize;

        samples.par_iter_mut().enumerate().for_each(|(i,sample)|
        {
            let mut s: f32 = 0.0;
            let t = i as f32 / self.sample_rate as f32;
            let mut h = 1;

            //ATTACK
            let envelope = if i < attackSamples {
                i as f32 / attackSamples as f32 // Linear attack envelope
            } else if i < num_samples - decaySamples {
                1.0 // Sustain phase
            }
            else
            {
                (num_samples - i) as f32 / decaySamples as f32
            };

            while h as f32 * frequency < nyquist_frequency
            {
                let harmonic_frequency = h as f32 * frequency;
                let harmonic_amplitude = self.amplitude / h as f32;
                s = s + (harmonic_amplitude * (t * harmonic_frequency * 2.0 * PI).sin());
                h+=1;
            }
            *sample = s * envelope;
        });
        return samples;
    }
}