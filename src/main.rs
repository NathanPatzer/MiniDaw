#![allow(non_snake_case)]
#![allow(unused_imports)]
extern crate hound;
mod sin;
mod m_note;
mod square;
mod triangle;
mod sawtooth;
mod noise;
mod chord;
mod writer;
mod waveforms;
use writer::*;
use waveforms::*;
use chord::*;
use noise::*;
use sawtooth::*;
use triangle::*;
use square::*;
use sin::*;
use m_note::*;
use std::{fs, arch::x86_64::_MM_EXCEPT_INVALID};
use midly::{Smf, TrackEventKind, MidiMessage, MetaMessage, num::{u15, u28}};
fn main() {
    let bytes = fs::read("MIDI/ZELDA_3.mid").unwrap();
    let smf = Smf::parse(&bytes).unwrap();
    let timing: midly::Timing = smf.header.timing;
    let mut ticks: u16 = 0;
    if let midly::Timing::Metrical(value) = timing
    {
        ticks = value.into();
    }
    let ticks_per_beat = ticks as u32;
    let BPM = 140;
    //let track = &smf.tracks[1];
    let notes: Note = Note::new();
    let s: Triangle = Triangle::new(4000.0);
    
    let mut allSamples: Vec<Vec<f32>> = Vec::new();
    let mut w: Writer = Writer::new();
    let mut i: i32 = -1;
    for track in smf.tracks
    {

        let curBuff: Vec<f32> = Vec::new();
        allSamples.push(curBuff);
        i+=1;
        for event in track.iter()
        {

            let mut durationON: midly::num::u28 = 0.into();
            let mut durationOFF: midly::num::u28 = 0.into();
            if let TrackEventKind::Midi { channel: _, message } = event.kind
            {
                
                if let MidiMessage::NoteOn { key: _, vel: _} = message
                {
                    
                    //CALCULATE SILENCE DURRATION
                    durationON = event.delta;
                    let totalSilence: u32 = (durationON - durationOFF).into();
                    let mut silence_durration: f32 = totalSilence as f32 / ticks_per_beat as f32;
                    silence_durration = (silence_durration / BPM as f32) * 60.0;
                    let num_samples = (44100 as f32 * silence_durration) as usize;
                    let samples: Vec<f32> = vec![0.0;num_samples];
                    for sample in samples
                    {
                        allSamples[i as usize].push(sample);
                    }
                }
                else if let MidiMessage::NoteOff { key, vel: _ } = message
                {
                    //CALCULATE NOTE DURRATION
                    durationOFF = event.delta;
                    let totalNote: u32 = (durationOFF - durationON).into();
                    let mut note_durration: f32 = totalNote as f32 / ticks_per_beat as f32;
                    note_durration = (note_durration / BPM as f32) * 60.0;
                    
                    //CALCULATE FREQUENCIES
                    let k: u8 = key.into();
                    let octave: i32 = (k as f32 / 12.0).floor() as i32;
                    let final_key = k as i32 - (octave * 12);
                    let snote: String = notes.note_int.get(&final_key).unwrap().clone();
                    let note_freq = notes.get(snote, octave as u8 - 2);
                    let currTone: Vec<f32> = s.gen(note_freq,note_durration);
                    //w.append(currTone);
                    for sample in currTone
                    {
                        allSamples[i as usize].push(sample);
                    }
                }
            }
            
        }
        
    }

let mut minSize: usize = allSamples[1].len();
let mut finalSamples: Vec<f32> = Vec::new();

for buff in allSamples.iter()
{
    if buff.len() != 0
    {
        if buff.len() < minSize
        {
            minSize = buff.len();
        }
    }
}
   
for i in 0..minSize
{
    let mut s: f32 = 0.0;
    for buff in allSamples.iter()
    {
        if buff.len() != 0
        {
            s = s + buff[i];
        }
    }
    finalSamples.push(s);
}

w.append(finalSamples);
w.write();
}