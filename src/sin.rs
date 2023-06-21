#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate hound;
use std::f32::consts::PI;

use rayon::prelude::{IntoParallelRefMutIterator, IndexedParallelIterator, ParallelIterator};

use crate::waveforms::Generate;

#[derive(Clone)]
pub struct Sin
{
    length: u32, //seconds
    sample_rate: u32
}

impl Sin
{
    pub fn new(len: u32) -> Sin
    {
        Sin {length: len,sample_rate: 44100}
    }

}

impl Generate for Sin
{
    fn gen(&self,frequency: f32, duration: f32) -> Vec<f32> {
        let num_samples = (self.sample_rate as f32 * duration) as usize;
        let mut samples: Vec<f32> = vec![0.0;num_samples];
        
        samples.par_iter_mut().enumerate().for_each(|(i,sample)|
        {
            let t = i as f32 / self.sample_rate as f32;
            let s = (t * frequency * 2.0 * PI).sin();
            *sample = s; 
        });
        return samples;
    }
}