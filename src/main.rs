// mod one;
mod two;
mod util;

fn main() {
    let path: String = format!("./res/day-2.txt");
    let solution = two::day_two(path);
    println!("{:?}", solution);
}
