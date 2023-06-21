#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::Triangle;
use crate::Sin;
use crate::Saw;
use crate::Square;
#[derive(Clone)]
pub enum WaveForm
{
    Triangle(Triangle),
    Sin(Sin),
    Saw(Saw),
    Square(Square)
}

pub trait Generate
{
    fn gen(&self,frequency: f32, duration: f32) -> Vec<f32>;
}

impl Generate for WaveForm
{
    fn gen(&self,frequency: f32, duration: f32) -> Vec<f32> {
        match self 
        {
            WaveForm::Saw(saw) => saw.gen(frequency,duration),
            WaveForm::Sin(sin) => sin.gen(frequency,duration),
            WaveForm::Square(square) => square.gen(frequency,duration),
            WaveForm::Triangle(tri) => tri.gen(frequency, duration)    
        }
    }
}