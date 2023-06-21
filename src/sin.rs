#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate hound;
use std::f32::consts::PI;

use rayon::prelude::{IntoParallelRefMutIterator, IndexedParallelIterator, ParallelIterator};

use crate::waveforms::Generate;

#[derive(Clone)]
pub struct Sin
{
    sample_rate: u32,
    amplitude: f32
}

impl Sin
{
    pub fn new(A: f32) -> Sin
    {
        Sin {sample_rate: 44100, amplitude: A}
    }

}

impl Generate for Sin
{
    fn gen(&self,frequency: f32, duration: f32) -> Vec<f32> {
        let num_samples = (self.sample_rate as f32 * duration) as usize;
        let mut samples: Vec<f32> = vec![0.0;num_samples];
        let attackTime: f32 = 0.05;
        let decayTime: f32 = 0.05;
        let decaySamples = (self.sample_rate as f32 * decayTime) as usize;
        let attackSamples = (self.sample_rate as f32 * attackTime) as usize;
        
        samples.par_iter_mut().enumerate().for_each(|(i,sample)|
        {
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
            let t = i as f32 / self.sample_rate as f32;
            let s = (t * frequency * 2.0 * PI).sin();
            *sample = s * envelope * self.amplitude; 
        });
        return samples;
    }
}