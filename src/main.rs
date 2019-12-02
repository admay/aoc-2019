use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn calc_fuel(f: f32) -> f32 {
    let mut total_fuel = 0.0;
    let mut cur_fuel = f;
    let mut done = false;
    while !done {
        let next_fuel: f32 = (cur_fuel/3.0).trunc() - 2.0;
        cur_fuel = next_fuel;
        if next_fuel > 0.0 {
            total_fuel += next_fuel;
        } else {
            done = true;
        }
    }
    total_fuel
}

fn day_one(path: &str) -> f32 {
    lines_from_file(path)
        .iter()
        .map(|x| x.parse::<f32>().unwrap())
        .map(|x| calc_fuel(x) )
        .sum()

}

fn main() {
    println!("{:?}", std::env::current_dir());
    let solution: f32 = day_one("./res/day-1.txt");
    println!("{}", solution);
}
