

use crate::{WaveForm, sin::Sin, triangle::{Triangle}, square::Square, sawtooth::Saw, noise::Noise};
pub struct ArgsChecker
{
    pub midi: String,
    pub name: String,
    pub BPM: i32,
    pub wave: WaveForm,
    pub transpose: i16
}

impl ArgsChecker
{
    pub fn new(args: Vec<String>) -> ArgsChecker
    {
        let mut BPM: i32 = 140;
        let mut midi: String = "".to_string();
        let mut name = "output".to_string();
        let mut WaveForm = WaveForm::Sin(Sin::new(4000.0));
        let mut transpose: i16 = 0;
        for i in 0..args.len()
        {
            if args[i] == "-i"
            {
                midi = args[i+1].to_string();
            }

            else if args[i] == "-n"
            {
                name = args[i+1].to_string();
            }
            else if args[i] == "-b"
            {
                BPM = args[i+1].parse::<i32>().unwrap();
            }
            else if args[i] == "-w"
            {
                if args[i+1].to_string() == "tri"
                {
                    WaveForm = WaveForm::Triangle(Triangle::new(4000.0));
                }
                else if args[i+1].to_string() == "square"
                {
                    WaveForm = WaveForm::Square(Square::new(4000.0));
                }
                else if args[i+1].to_string() == "saw"
                {
                    WaveForm = WaveForm::Saw(Saw::new(4000.0));
                }
                else if args[i+1].to_string() == "sin"
                {
                    WaveForm = WaveForm::Sin(Sin::new(4000.0));
                }
                else if args[i].to_string() == "noise"
                {
                    WaveForm = WaveForm::Noise(Noise::new(4000.0));
                }
            }
            else if args[i] == "-t" {
                transpose = args[i+1].parse::<i16>().unwrap();
            }
        }       

        ArgsChecker { midi: midi, name: name,BPM: BPM, wave: WaveForm,transpose: transpose }
    }

    
}

pub type Args = ArgsChecker;