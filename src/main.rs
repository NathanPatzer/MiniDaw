#![allow(non_snake_case)]
#![allow(unused_imports)]
extern crate hound;
mod sin;
mod square;
mod triangle;
mod sawtooth;
mod noise;
mod writer;
mod DSP;
mod ArgsChecker;
mod waveforms;
mod MidiParser;
use std::f32::consts::E;

use MidiParser::*;
use num_complex::Complex;
use writer::*;
use DSP::*;
use waveforms::*;
use noise::*;
use sawtooth::*;
use triangle::*;
use square::*;
use sin::*;
use ArgsChecker::*;

fn main() {
    let plainargs: Vec<String> = std::env::args().collect();
    let args: Args = Args::new(plainargs);
    let midi_parser: MidiParser::MidiParser = MidiParser::MidiParser::new(args.midi, args.BPM, args.wave);
    let final_samples: Vec<f32> = midi_parser.parse(args.transpose);
    let mut w: Writer = Writer::new(args.name);

    let sin: Sin = Sin::new(2000.0);
    let test_samples: Vec<f32> = sin.gen(500.0, 10.0);

    let mut currBuf: Vec<Complex<f32>> = Vec::new();
    for i in 0..test_samples.len()
    {
        if i % 4096 == 0 && i != 0
        {
            let mut buff = currBuf.clone();
            DSP::DSP::FFT(&mut buff);
            println!("{}",DSP::DSP::find_freq(buff, 44100));
            currBuf.clear();
            currBuf.push(Complex::new(test_samples[i], 0.0));
        }
        else {
            currBuf.push(Complex::new(test_samples[i], 0.0));
        }
    }
    w.append(final_samples.clone());
    w.write(); 
}