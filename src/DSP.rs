#![allow(non_snake_case)]
#![allow(dead_code)]
use std::{f32::consts::PI};
use num_complex::Complex;
//COOLEY TURKEY ALGORITHM
pub fn FFT(signal: &mut Vec<Complex<f32>>)
{
    let n: usize = signal.len();

    if n <= 1{return;}
    let mut even: Vec<Complex<f32>> = Vec::new();
    let mut odd: Vec<Complex<f32>> = Vec::new();
    for i in 0..n{
        if i % 2 == 0{
            even.push(signal[i]);
        }
        else{
            odd.push(signal[i]);
        }
    }
    //let mut even: Vec<Complex<f32>> = signal.iter().enumerate().filter(|(index, _)| index % 2 == 0).map(|(_, value)| *value).collect();
    //let mut odd: Vec<Complex<f32>> = signal.iter().enumerate().filter(|(index, _)| (index + 1) % 2 == 0).map(|(_, value)| *value).collect();
    FFT(&mut even);
    FFT(&mut odd);

    for k in 0..n/2
    {
        let t: Complex<f32> = Complex::new(0.0, -2.0 * PI * (k as f32/n as f32)).exp() * odd[k];
        signal[k] = even[k] + t;
        signal[k + (n/2)] = even[k] - t;
    }
}

//FINDS DOMINANT FREQUENCY
pub fn find_freq(fft_result: Vec<Complex<f32>>, sample_rate: u32) -> f32
{
    let N: usize = fft_result.len();
    let bin_size = sample_rate as f32 / N as f32;
    let mut max_magnitude: f32 = 0.0;
    let mut dominant_frequency: f32 = 0.0;
    
    for k in 0..N/2
    {
        let curr_mag: f32 = fft_result[k].norm();

        if curr_mag > max_magnitude
        {
            max_magnitude = curr_mag;
            dominant_frequency = k as f32 * bin_size;
        }
    }

    return dominant_frequency;
}

pub fn IFFT(_input: &mut Vec<Complex<f32>>)
{
    
}  