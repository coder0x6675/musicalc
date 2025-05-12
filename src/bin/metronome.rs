
fn main() {

    let bpm: u32 = std::env::args()
        .nth(1)
        .expect("No BPM specified")
        .parse()
        .expect("The argument should be a tempo/BPM");

    let period = 60.0 / bpm as f64;
    let mut tick: u8 = 0;

    loop {

        if tick % 4 == 0 {
            println!("TICK");
        }
        else {
            println!("TOCK");
        }

        std::thread::sleep(std::time::Duration::from_secs_f64(period));
        tick = tick.wrapping_add(1);
    }
}

