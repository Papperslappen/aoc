use clap::Parser;
use std::io;

mod dec01;
mod dec02;
mod dec03;
mod dec04;
mod dec05;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 5)]
    date: u8,
}

fn main() {
    let args = Args::parse();
    let result = match args.date {
        1 => dec01::solution(read_input()),
        2 => dec02::solution(read_input()),
        3 => dec03::solution(read_input()),
        4 => dec04::solution(read_input()),
        5 => dec05::solution(read_input()),
        _ => {
            panic!()
        }
    };
    println!("Result A: {} \n Result B: {}", result.0, result.1);
}

fn read_input() -> Vec<String> {
    let stdin = io::stdin();
    stdin.lines().map(|s| s.unwrap()).collect()
}
