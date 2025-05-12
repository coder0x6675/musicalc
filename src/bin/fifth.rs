
use std::ops::Add;


const NOTES: &[&str] = &["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const NOTE_COUNT: usize = NOTES.len();


fn main() {

    let note = std::env::args()
        .nth(1)
        .expect("The first argument should be a note")
        .to_uppercase();
    let note = NOTES
        .iter()
        .position(|&x| x == note)
        .expect("The first argument should be a valid note")
        .add(NOTE_COUNT);

    let root_note   = (note + 0) % NOTE_COUNT;
    let lower_fifth = (note - 7) % NOTE_COUNT;
    let upper_fifth = (note + 7) % NOTE_COUNT;
    let to_minor    = (note - 3) % NOTE_COUNT;
    let to_major    = (note + 3) % NOTE_COUNT;
	
    println!("Root note: {}",   NOTES[root_note]);
    println!("- Lower 5th: {}", NOTES[lower_fifth]);
    println!("- Upper 5th: {}", NOTES[upper_fifth]);
    println!("- To minor: {}",  NOTES[to_minor]);
    println!("- To major: {}",  NOTES[to_major]);
}

