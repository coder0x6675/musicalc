
use std::ops::Add;


// Using standard tuning (A4)
const STD_TUNE: f64 = 440.0;
const STD_NOTE: f64 = 57.0;
const NOTES: &[&str] = &["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];


fn main() {

    let str_note = std::env::args()
        .nth(1)
        .expect("The first argument should be a note")
        .to_uppercase();

    let is_sharp = '#' == str_note
        .chars()
        .nth(1)
        .expect("The first argument is too short to be a valid note");
    let separator = if is_sharp {2} else {1};

    let octave: i32 = str_note[separator..]
        .parse()
        .expect("The octave is invalid");

    let note = TryInto::<i32>::try_into(NOTES
        .iter()
        .position(|&x| x == &str_note[..separator])
        .expect("The first argument should be a valid note"))
        .unwrap()
        .add(octave * NOTES.len() as i32);

    let distance = note as f64 - STD_NOTE;
    let frequency = STD_TUNE * f64::powf(2.0, distance / 12.0);

    println!("{str_note} = {frequency:.2} hz");
}

