
const SEPARATOR: &str = "    ";
const NOTE_RANGE: u32 = 9;


fn main() {

    let bpm: u32 = std::env::args()
        .nth(1)
        .expect("No BPM specified")
        .parse()
        .expect("The argument should be a tempo/BPM");

    println!(" NOTE{}  WHOLE{}TRIPLET{} DOTTED", SEPARATOR, SEPARATOR, SEPARATOR);

    for i in 0..NOTE_RANGE {

        let exponent = u32::pow(2, i);
        let whol_ms = 240_000.0 / (bpm * exponent) as f64;
        let trip_ms = 160_000.0 / (bpm * exponent) as f64;
        let dott_ms = 300_000.0 / (bpm * exponent) as f64;

        println!("1/{: >3}{}{: >7.2}{}{: >7.2}{}{: >7.2}",
            exponent,
            SEPARATOR,
            whol_ms,
            SEPARATOR,
            trip_ms,
            SEPARATOR,
            dott_ms,
        );
    }
}

