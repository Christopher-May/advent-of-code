use std::fs::File;
use std::io::{prelude::*, BufReader};

fn calc_fuel(coord: i32) -> i32 {
    (coord / 3) - 2
}

fn calc_fuel2(coord: i32) -> i32 {
    let mut total_cost = 0;
    let mut cur_cost = (coord / 3) - 2;

    while cur_cost >= 0 {
        total_cost += cur_cost;
        cur_cost = (cur_cost / 3) - 2;
    }

    total_cost
}

fn calc_modules(calc: &dyn Fn(i32) -> i32) -> i32 {
    let mut fuel = 0;

    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(string) = line {
                if let Ok(num) = string.parse::<i32>() {
                    fuel += calc(num);
                }
            }
        }
    }
    fuel
}

fn main() {
    let modules = calc_modules(&calc_fuel);
    println!(" fuel needed for part 1 { }", modules);

    let modules = calc_modules(&calc_fuel2);
    println!(" fuel needed for part 2 { }", modules);
}
