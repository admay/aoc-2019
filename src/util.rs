use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn split_lines_from_file(filename: String) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("no such file");
    let splits: Vec<&str> = contents
        .trim()
        .split(",")
        .collect();
    let splits: Vec<String> = splits.iter().map(|x| x.to_string()).collect();
    splits
}
