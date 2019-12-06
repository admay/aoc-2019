#[aoc_generator(day6)]
fn parse_input(input: &str) -> i32 {
    let x: Vec<Vec<&str>> = input
        .lines()
        .map(|x| {
            x.split(")").collect::<Vec<&str>>();
        })
        .collect();

    for line in x {
        println!("{:?}", line);
    }

    0
}

#[aoc(day6, part1)]
fn solve_p1(x: &i32) -> i32 {
    println!("{:?}", x);
    0
}
