#[derive(Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub enum Orientation {
    CLOCKWISE,
    COUNTERCLOCKWISE,
    COLINEAR,
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    distance: i64
}

pub type Point = (i64, i64);

pub type Line = (Point, Point);

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            line
                .split(",")
                .map(|i| Instruction {
                    direction: match i.chars().next().unwrap() {
                        'U' => Direction::UP,
                        'D' => Direction::DOWN,
                        'L' => Direction::LEFT,
                        'R' => Direction::RIGHT,
                        _ => unreachable!(),
                    },
                    distance: i[1..].parse().unwrap(),
                })
            .collect()
        })
        .collect()
}

fn gen_path(wire: &[Instruction]) -> Vec<Point> {
    let mut cur_pt: Point = (0, 0);
    let mut path: Vec<Point> = vec![cur_pt];
    for ins in wire {
        match ins.direction {
            Direction::UP => path.push((cur_pt.0, cur_pt.1 + ins.distance)),
            Direction::DOWN => path.push((cur_pt.0, cur_pt.1 - ins.distance)),
            Direction::LEFT => path.push((cur_pt.0 - ins.distance, cur_pt.1)),
            Direction::RIGHT => path.push((cur_pt.0 + ins.distance, cur_pt.1)),
        }
        cur_pt = *path.last().unwrap();
    }
    path
}

fn find_intersection(p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> Option<Point> {
    let p0_x = p0.0 as f64;
    let p0_y = p0.1 as f64;

    let p1_x = p1.0 as f64;
    let p1_y = p1.1 as f64;

    let p2_x = p2.0 as f64;
    let p2_y = p2.1 as f64;

    let p3_x = p3.0 as f64;
    let p3_y = p3.1 as f64;

    let s1_x = p1_x - p0_x;
    let s1_y = p1_y - p0_y;
    let s2_x = p3_x - p2_x;
    let s2_y = p3_y - p2_y;

    let det = -s2_x * s1_y + s1_x * s2_y;
    let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / det;
    let t = (s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / det;

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        let ix = (p0_x + (t * s1_x)) as i64;
        let iy = (p0_y + (t * s1_y)) as i64;
        Some((ix, iy))
    }
    else {
        None
    }
}

fn find_intersections(path_1: Vec<Point>, path_2: Vec<Point>) -> Vec<(u64, Point)> {
    let mut intersections: Vec<(u64, Point)> = vec![];
    let mut steps_a = 0;
    for (p0, p1) in path_1.iter().zip(path_1[1..].iter()) {
        steps_a += manhattan_distance(p0, p1);

        let mut steps_b = 0;
        for (p2, p3) in path_2.iter().zip(path_2[1..].iter()) {
            steps_b += manhattan_distance(p2, p3);
            if let Some(o) = find_intersection(p0, p1, p2, p3) {
                if o != (0, 0) {
                    let dist = steps_a + steps_b
                        - manhattan_distance(p1, &o)
                        - manhattan_distance(p3, &o);
                    intersections.push((dist, o));
                }
            }
        }
    }
    intersections
}

fn manhattan_distance(p: &Point, q: &Point) -> u64 {
    (p.0 - q.0).abs() as u64 + (p.1 - q.1).abs() as u64
}

#[aoc(day3, part1)]
pub fn solve_p1(input: &Vec<Vec<Instruction>>) -> u64 {
    let wire_1 = gen_path(&input[0]);
    let wire_2 = gen_path(&input[1]);

    let intersections = find_intersections(wire_1, wire_2);
    let closest_intersect = intersections
        .iter()
        .min_by(|p, q| {
            manhattan_distance(
                &(0, 0), &p.1)
                .cmp(&manhattan_distance(&(0, 0), &q.1))
        })
        .unwrap();

    manhattan_distance(&(0, 0), &closest_intersect.1)
}

#[aoc(day3, part2)]
pub fn solve_p2(input: &Vec<Vec<Instruction>>) -> u64 {
    let wire_1 = gen_path(&input[0]);
    let wire_2 = gen_path(&input[1]);

    let intersections = find_intersections(wire_1, wire_2);
    let closest_intersect = intersections
        .iter()
        .min_by_key(|(dist, _)| dist)
        .unwrap();

    closest_intersect.0
}
