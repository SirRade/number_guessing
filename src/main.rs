extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn main() {
    let rn = get_random();
    let tries = 10;
    for i in 0..tries {
        println!("[{}/{}] Enter your guess: ", i, tries);
        let input = get_input();
        match (input < rn, input > rn) {
            (true, false) => println!("Too small"),
            (false, true) => println!("Too big"),
            (false, false) => { println!("Correct!"); break; },
            _ => panic!("input is both too high and too low"),
        }
    }
}

fn get_random() -> i32 {
    let range = Range::new(0, 100);
    let mut rng = rand::thread_rng();

    range.ind_sample(&mut rng)
}

fn get_input() -> i32 {
    let reader = std::io::stdin();
    let mut input = String::new();

    reader
        .read_line(&mut input)
        .expect("Couldn't read line");

    input
        .trim()
        .parse::<i32>()
        .expect("Couldn't convert input into number")
}
