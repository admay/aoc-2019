#[aoc(day1, part1)]
pub fn solve_p1(input: &str) -> f64 {
    input
        .lines()
        .map(|x| x.parse::<f64>().unwrap())
        .map(|x| (x / 3.0).trunc() - 2.0)
        .sum()
}

pub fn calc_fuel(f: f64) -> f64 {
    let mut total_fuel = 0.0;
    let mut cur_fuel = f;
    let mut done = false;
    while !done {
        let next_fuel: f64 = (cur_fuel / 3.0).trunc() - 2.0;
        cur_fuel = next_fuel;
        if next_fuel > 0.0 {
            total_fuel += next_fuel;
        } else {
            done = true;
        }
    }
    total_fuel
}

#[aoc(day1, part2)]
pub fn solve_p2(input: &str) -> f64 {
    input
        .lines()
        .map(|x| x.parse::<f64>().unwrap())
        .map(calc_fuel)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_sample1() {
            assert_eq!(solve_p1("12"), 2.0)
        }

    #[test]
    fn p1_sample2() {
            assert_eq!(solve_p1("14"), 2.0)
        }

    #[test]
    fn p1_sample3() {
            assert_eq!(solve_p1("1969"), 654.0)
        }

    #[test]
    fn p1_sample4() {
            assert_eq!(solve_p1("100756"), 33583.0)
        }

    #[test]
    fn p2_sample1() {
            assert_eq!(solve_p2("12"), 2.0)
        }

    #[test]
    fn p2_sample2() {
            assert_eq!(solve_p2("1969"), 966.0)
        }

    #[test]
    fn p2_sample3() {
            assert_eq!(solve_p2("100756"), 50346.0)
        }
}
