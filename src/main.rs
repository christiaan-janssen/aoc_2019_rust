use std::fs;

mod puzzle_01;
use puzzle_01::*;

fn main() {
    let input= fs::read_to_string("../input/puzzle01.txt")
        .expect("Cant read the file!");
    let masses: Vec<&str> = input.split("\n").collect();

    let mut total_fuel: i32 = 0;
    for w in masses {
        total_fuel += one_a(w.parse::<i32>().unwrap())
    }
    println!("{}",total_fuel);
}
