use std::{fs, arch::x86_64::_MM_EXCEPT_INVALID};
use midly::{Smf, TrackEventKind, MidiMessage, MetaMessage, num::{u15, u28, u7}};

use crate::{waveforms::{WaveForm, Generate}};

pub struct MidiParser
{
    wave: WaveForm,
    midi_file: String,
    BPM: i32
}

impl MidiParser
{
    pub fn new(midi_file: String, BPM: i32, wave: WaveForm) -> MidiParser
    {
        MidiParser { wave: wave, midi_file: midi_file, BPM: BPM}
    }

    pub fn parse(&self,transpose: i16) -> Vec<f32>
    {
        let bytes = fs::read(self.midi_file.clone()).unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        let ticks_per_beat: u32 = MidiParser::get_ticks_per_beat(smf.header.timing);
        let mut i: i32 = 0;
        let mut allSamples: Vec<Vec<f32>> = Vec::new();

        for track in smf.tracks
        {
            let curBuff: Vec<f32> = Vec::new();
            allSamples.push(curBuff);
            
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
                        let silence_durration: f32 = MidiParser::get_durration(durationON, durationOFF,ticks_per_beat,self.BPM);
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
                        let note_durration: f32 = MidiParser::get_durration(durationOFF, durationON, ticks_per_beat, self.BPM);
                        let u8_key: u8 = key.into();
                        let i16_key: i16 = u8_key as i16;
                        let currTone: Vec<f32> = self.wave.gen(MidiParser::get_frequency(i16_key + (transpose * 12)),note_durration);
                        for sample in currTone
                        {
                            allSamples[i as usize].push(sample);
                        }
                    }
                }
                
            }
            i+=1;
        }
    
    let mut finalSamples: Vec<f32> = Vec::new();
       
    for i in 0..MidiParser::get_max_samples(&allSamples)
    {
        let mut s: f32 = 0.0;
        for buff in allSamples.iter()
        {
            if buff.len() != 0 && i < buff.len()
            {
                s = s + buff[i];
            }
        }
        finalSamples.push(s);
    }

    return finalSamples;
    }

    fn get_frequency(note: i16) -> f32
    {
        
        return 440.0 * 2.0_f32.powf((note as f32 - 69.0)/12.0);
    }

    fn get_durration(durationON: midly::num::u28,durationOFF: midly::num::u28,ticks_per_beat: u32,BPM: i32) -> f32
    {
        let totalSilence: u32 = (durationON - durationOFF).into();
        let silence_durration: f32 = totalSilence as f32 / ticks_per_beat as f32;
        (silence_durration / BPM as f32) * 60.0
    }

    fn get_max_samples(allSamples: &Vec<Vec<f32>>) -> usize
    {
        let mut maxSize: usize = 0;
        for buff in allSamples.iter()
        {
            if buff.len() != 0
            {
                if buff.len() > maxSize
                {
                    maxSize = buff.len();
                }
            }
        }
        return maxSize;
    }

    fn get_ticks_per_beat(timing: midly::Timing) -> u32
    {
        let mut ticks: u16 = 0;
        if let midly::Timing::Metrical(value) = timing
        {
            ticks = value.into();
        }
        ticks as u32
    }
}