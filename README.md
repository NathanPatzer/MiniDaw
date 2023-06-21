# MiniDaw

This is a miniture DAW built in rust. It can generate different waveforms such as sine, square, triangle, sawtooth, and noise. This program can also read in MIDI files and play a melody with those waveforms!

## Setup
1. Install Rust: Make sure you have Rust installed on your system. You can download it from the official website: [https://www.rust-lang.org](https://www.rust-lang.org)

2. Clone the repository: Clone this repository to your local machine using Git.
```
git clone https://github.com/NathanPatzer/MiniDaw.git
```
3. Navigate to the project directory: Use the `cd` command to go into the cloned repository's directory.
```
cd MiniDaw
```

## Usage
Build the project: Run the following command to build the MiniDaw.
```
cargo build --release
.\target\release\Audio.exe [ARGS]
```
or for a debug version
```
cargo build
.\target\debug\Audio.exe [ARGS]
```

- `-n [file_name]`: Specifies the name of the wav file that will be exported. Default to `output.wav`
- `-i [input_file]`: Specifies the input MIDI file containing the musical information. Mandatory argument.
- `-b [bpm]`: Specifies the bpm the midi will be played at
- `-w [waveform]`: Specifies the waveform that will be generated. [sin,square,tri,saw]

- `-t [transpose]`: Specifies the how many octaves to transpose the midi file by. Can be negative or positive.

## Generated WAV's
<iframe width="100%" height="300" scrolling="no" frameborder="no" allow="autoplay" src="https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1546520794%3Fsecret_token%3Ds-0WxGahKZzUN&color=%23ff5500&auto_play=false&hide_related=false&show_comments=true&show_user=true&show_reposts=false&show_teaser=true&visual=true"></iframe><div style="font-size: 10px; color: #cccccc;line-break: anywhere;word-break: normal;overflow: hidden;white-space: nowrap;text-overflow: ellipsis; font-family: Interstate,Lucida Grande,Lucida Sans Unicode,Lucida Sans,Garuda,Verdana,Tahoma,sans-serif;font-weight: 100;"><a href="https://soundcloud.com/itspatzy" title="Patzy" target="_blank" style="color: #cccccc; text-decoration: none;">Patzy</a> · <a href="https://soundcloud.com/itspatzy/still-alive-1/s-0WxGahKZzUN" title="Still Alive" target="_blank" style="color: #cccccc; text-decoration: none;">Still Alive</a></div>