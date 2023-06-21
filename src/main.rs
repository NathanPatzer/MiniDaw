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
use MidiParser::*;
use writer::*;
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
    w.append(final_samples);
    w.write();
}