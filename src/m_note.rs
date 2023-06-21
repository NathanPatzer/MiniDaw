
use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct m_note
{
    notes: HashMap<String,f32>,
    pub note_vec: Vec<String>,
    pub note_int: HashMap<i32,String>
}

impl m_note
{
    pub fn new() -> Note
    {
        let mut notes: HashMap<String,f32> = HashMap::new();
        let mut note_vec: Vec<String> = Vec::new();
        let mut note_int: HashMap<i32,String> = HashMap::new();
        notes.insert("C".to_string(), 16.35);
        notes.insert("C#".to_string(), 17.32);
        notes.insert("D".to_string(), 18.35);
        notes.insert("D#".to_string(), 19.45);
        notes.insert("E".to_string(), 20.60);
        notes.insert("F".to_string(), 21.83);
        notes.insert("F#".to_string(), 23.12);
        notes.insert("G".to_string(),24.50);
        notes.insert("G#".to_string(), 25.96);
        notes.insert("A".to_string(), 27.50);
        notes.insert("A#".to_string(), 29.14);
        notes.insert("B".to_string(), 30.87);

        note_vec.push("C".to_string());
        note_vec.push("C#".to_string());
        note_vec.push("D".to_string());
        note_vec.push("D#".to_string());
        note_vec.push("E".to_string());
        note_vec.push("F".to_string());
        note_vec.push("F#".to_string());
        note_vec.push("G".to_string());
        note_vec.push("G#".to_string());
        note_vec.push("A".to_string());
        note_vec.push("A#".to_string());
        note_vec.push("B".to_string());

        note_int.insert(0, "C".to_string());
        note_int.insert(1, "C#".to_string());
        note_int.insert(2, "D".to_string());
        note_int.insert(3, "D#".to_string());
        note_int.insert(4, "E".to_string());
        note_int.insert(5, "F".to_string());
        note_int.insert(6, "F#".to_string());
        note_int.insert(7, "G".to_string());
        note_int.insert(8, "G#".to_string());
        note_int.insert(9, "A".to_string());
        note_int.insert(10, "A#".to_string());
        note_int.insert(11, "B".to_string());

        Note{notes: notes, note_vec: note_vec,note_int: note_int}
    }

    pub fn get(&self,note: String, octave: u8) -> f32
    {
        let base: &f32 = self.notes.get(&note).unwrap();
        base * 2_f32.powf(octave as f32)
    }
}

pub type Note = m_note;