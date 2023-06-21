#![allow(non_snake_case)]
#![allow(dead_code)]
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
pub struct Writer
{
    writer: hound::WavWriter<std::io::BufWriter<std::fs::File>>,
    pub samples: Vec<f32>
}

impl Writer
{
    pub fn new(opt: String) -> Writer
    {
        let spec: hound::WavSpec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let channels:Vec<f32> = Vec::new();
        let writer: hound::WavWriter<std::io::BufWriter<std::fs::File>> = hound::WavWriter::create("WAV/".to_owned()+&opt + ".wav", spec).unwrap();
        Writer { writer: writer, samples: channels }
    }

    pub fn append(&mut self,samples: Vec<f32>)
    {
        for i in 0..samples.len()
        {
            self.samples.push(samples[i]);
        }
    }

    pub fn write(&mut self)
    {
            for i in 0..self.samples.len()
            {
                self.writer.write_sample(self.samples[i] as i16).unwrap();
            }
    }

    pub fn append_chord(&mut self, s1: Vec<f32>, s2: Vec<f32>, s3: Vec<f32>)
    {
        for i in 0..s1.len()
        {
            let sample = s1[i] + s2[i] + s3[i];
            self.writer.write_sample(sample as i16).unwrap();
        }
    }
}