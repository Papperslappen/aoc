use clap::Parser;
use std::{io, time::Instant};

mod dec01;
mod dec02;
mod dec03;
mod dec04;
mod dec05;
mod dec06;
mod dec07;
mod dec08;
mod dec09;
mod dec10;
mod dec11;
mod dec12;
mod dec13;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 13)]
    date: u8,
}

fn main() {
    let args = Args::parse();
    let now = Instant::now();
    let result = match args.date {
        1 => dec01::solution(read_input()),
        2 => dec02::solution(read_input()),
        3 => dec03::solution(read_input()),
        4 => dec04::solution(read_input()),
        5 => dec05::solution(read_input()),
        6 => dec06::solution(read_input()),
        7 => dec07::solution(read_input()),
        8 => dec08::solution(read_input()),
        9 => dec09::solution(read_input()),
        10 => dec10::solution(read_input()),
        11 => dec11::solution(read_input()),
        12 => dec12::solution(read_input()),
        13 => dec13::solution(read_input()),
        _ => {
            panic!()
        }
    };
    let time = now.elapsed();
    println!("Result A: {} \n Result B: {}", result.0, result.1);
    println!("It took {} ms", time.as_millis());
}

fn read_input() -> Vec<String> {
    let stdin = io::stdin();
    stdin.lines().map(|s| s.unwrap()).collect()
}
