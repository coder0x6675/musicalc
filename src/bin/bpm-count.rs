 
use std::io::Write;
use std::collections::VecDeque;


const QUEUE_MAX: usize = 12;


fn await_keypress() {
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Error while waiting for keypress");
}


fn main() {

    let mut time;
    let mut bps;
    let mut bpm;
    let mut stable = false;
    let mut time_queue: VecDeque<std::time::Instant> = VecDeque::with_capacity(QUEUE_MAX);

    println!("Press <ENTER> to start counting...");
    await_keypress();
    time_queue.push_back(std::time::Instant::now());

    print!("BPM: 0");
    std::io::stdout()
        .flush()
        .expect("Could not flush stdout");

    loop {
        await_keypress();

        if time_queue.len() >= QUEUE_MAX {
            if !stable {
                println!("STABLE!");
                stable = true;
            }
            time = time_queue
                .pop_front()
                .unwrap()
                .elapsed()
                .as_secs_f64();
        }
        else {
            time = time_queue
                .front()
                .unwrap()
                .elapsed()
                .as_secs_f64();
        }

        time_queue.push_back(std::time::Instant::now());

        bps = time_queue.len() as f64 / time;
        bpm = bps * 60.0;

        print!("BPM: {:.0}", bpm);
        std::io::stdout()
            .flush()
            .expect("Could not flush stdout");
    }
}

