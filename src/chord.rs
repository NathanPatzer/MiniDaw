#![allow(non_snake_case)]
#![allow(dead_code)]
use std::collections::HashMap;
use crate::{Note, waveforms::{WaveForm, Generate}};

pub struct Chord
{
    notes: Note,
    wave: WaveForm
}

impl Chord
{
    pub fn new(note: Note, wave: WaveForm) -> Chord
    {
        Chord { notes: note, wave: wave }
    }

    pub fn gen(&self, root: String,octave: u8,duration: f32,isMajor: bool) -> Vec<f32>
    {
        let num_notes = self.notes.note_vec.len();
        let mut thrup: (f32,f32,f32) = (0.0,0.0,0.0);
        for i in 0..num_notes
        {
            if self.notes.note_vec[i] == root
            {
                let mut sOct = octave;
                let mut tOct = octave;
                let mut sidx;
                let mut tidx;
                if isMajor
                {
                    sidx = i + 4;
                    tidx = i + 7;
                }
                else {
                    sidx = i + 3;
                    tidx = i + 7;
                }

                if sidx >= num_notes
                {
                    sidx = sidx - num_notes;
                    sOct+=1;
                }
                if tidx >= num_notes
                {
                    tidx = tidx - num_notes;
                    tOct+=1;
                }
                let s: &String = &self.notes.note_vec[sidx];
                let t: &String = &self.notes.note_vec[tidx];
                thrup = (self.notes.get(root, octave),self.notes.get(s.to_string(), sOct),self.notes.get(t.to_string(), tOct));
                break;
            }
        }

        return self.gen_chord(thrup,duration);
    }

    fn gen_chord(&self, frequencies: (f32,f32,f32),duration: f32) -> Vec<f32>
    {
        let s1: Vec<f32> = self.wave.gen(frequencies.0, duration);
        let s2: Vec<f32> = self.wave.gen(frequencies.1, duration);
        let s3: Vec<f32> = self.wave.gen(frequencies.2, duration);
        let mut r: Vec<f32> = Vec::new();

        for i in 0..s1.len()
        {
            r.push(s1[i] + s2[i] + s3[i]);
        }

        return r;
    }
}