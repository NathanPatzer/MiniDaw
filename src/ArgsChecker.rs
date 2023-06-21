use crate::{WaveForm, sin::Sin, triangle::{self, Triangle}, square::Square, sawtooth::Saw};
pub struct ArgsChecker
{
    pub midi: String,
    pub name: String,
    pub BPM: i32,
    pub wave: WaveForm
}

impl ArgsChecker
{
    pub fn new(args: Vec<String>) -> ArgsChecker
    {
        let mut BPM: i32 = 140;
        let mut midi: String = "".to_string();
        let mut name = "output".to_string();
        let mut WaveForm = WaveForm::Sin(Sin::new(2000.0));
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
            }
        }       

        ArgsChecker { midi: midi, name: name,BPM: BPM, wave: WaveForm }
    }

    
}

pub type Args = ArgsChecker;