use super::util;

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

pub fn day_one(path: String) -> f32 {
    util::lines_from_file(path)
        .iter()
        .map(|x| x.parse::<f32>().unwrap())
        .map(|x| calc_fuel(x) )
        .sum()

}
