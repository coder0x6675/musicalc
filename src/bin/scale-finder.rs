
const NOTES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const NOTE_COUNT: usize = NOTES.len();
const SCALES: [(&str, &[usize]); 8] = [
    ("Major",            &[0, 2, 4, 5, 7, 9, 11]),
    ("Minor (natural)",  &[0, 2, 3, 5, 7, 8, 10]),
    ("Minor (harmonic)", &[0, 2, 3, 5, 7, 8, 11]),
    ("Minor (melodic)",  &[0, 2, 3, 5, 7, 9, 11]),
    ("Phrygian",         &[0, 1, 3, 5, 7, 8, 10]),
    ("Blues",            &[0, 3, 5, 6, 7, 10]),
    ("Whole tone",       &[0, 2, 4, 6, 8, 10]),
    ("Pentatonic",       &[0, 2, 4, 7, 9]),
    //("chromatic",        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
];


fn main() {

    let in_notes: Vec<usize> = std::env::args()
        .skip(1)
        .map(|arg| NOTES
             .iter()
             .position(|&x| x == arg.to_uppercase())
             .expect("Arguments contains an invalid note"))
        .collect();

    for (scale_name, note_intervals) in SCALES.iter() {
        for root_note in 0..NOTE_COUNT {

            let scale_notes: Vec<usize> = note_intervals
                .iter()
                .map(|&interval| (root_note + interval) % NOTE_COUNT)
                .collect();

            if in_notes.iter().all(|&note| scale_notes.contains(&note)) {
                println!("{:2} {scale_name}", NOTES.get(root_note).unwrap());
            }
        }
    }
}

