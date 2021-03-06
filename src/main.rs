use std::fs;

mod puzzle_01;
use puzzle_01::*;

fn main() {
    let input= fs::read_to_string("../input/puzzle01.txt")
        .expect("Cant read the file!");
    let masses: Vec<&str> = input.split("\n").collect();

    let mut total_fuel: i32 = 0;
    for w in &masses {
        total_fuel += calculate_fuel_for_mass(w.parse::<i32>().unwrap())
    }
    println!("Puzzle 01 A, mass: {}",total_fuel);

    total_fuel = 0;
    for w in &masses {
        total_fuel += calculate_fuel_for_fuel(w.parse::<i32>().unwrap(), 0)
    }
    println!("Puzzel 01 B, mass: {}", total_fuel);
}
