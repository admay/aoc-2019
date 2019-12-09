use std::collections::{HashMap, VecDeque};

#[aoc_generator(day6, part1)]
fn parse_input_p1(input: &str) -> HashMap<String, Vec<String>> {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();

    input.trim().split("\n").for_each(|line| {
            let mut objs = line.trim().split(")");
            let parent = objs.next().unwrap().to_string();
            let child = objs.next().unwrap().to_string();
            orbits.entry(parent).or_default().push(child);
        });

    orbits
}

#[aoc(day6, part1)]
fn solve_p1(orbits: &HashMap<String, Vec<String>>) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back(("COM", 1));
    let mut count: u32 = 0;

    while queue.len() > 0 {
        let (parent, cur_count) = queue.pop_back().unwrap();
        if let Some(childs) = orbits.get(parent) {
            for child in childs {
                count += cur_count;
                queue.push_back((child, cur_count + 1));
            }
        }
    }
    count
}

#[aoc_generator(day6, part2)]
fn parse_input_p2(input: &str) -> HashMap<String, String> {
    let mut orbits: HashMap<String, String> = HashMap::new();

    input.trim().split("\n").for_each(|line| {
        let mut objs = line.trim().split(")");
        let parent = objs.next().unwrap().to_string();
        let child = objs.next().unwrap().to_string();
        orbits.entry(child).or_insert(parent);
    });

    orbits
}

fn first_common_element(from: Vec<String>, to: Vec<String>) -> Option<(usize, usize)> {
    let mut hm: HashMap<String, usize> = HashMap::new();
    for (i, item) in to.into_iter().enumerate() {
        hm.insert(item, i);
    }
    for (i, f) in from.iter().enumerate() {
        if let Some(j) = hm.get(f) {
            return Some((i, *j));

        }
    }
    None
}

fn parents(m: HashMap<String, String>, from: &str) -> Vec<String> {
    let mut cur = from;
    let mut parents = Vec::new();
    while cur != "COM" {
        if let Some(parent) = m.get(cur) {
            parents.push(parent.clone());
            cur = parent;
        }
    }
    parents
}

fn min_orbital_seq(orbits: HashMap<String, String>, from: &str, to: &str) -> usize {
    let from_parents = parents(orbits.clone(), from);
    let to_parents = parents(orbits.clone(), to);

    let (i, j) = first_common_element(from_parents, to_parents).unwrap();
    i + j
}

#[aoc(day6, part2)]
fn solve_p2(orbits: &HashMap<String, String>) -> usize {
    let from = "YOU";
    let to = "SAN";
    min_orbital_seq(orbits.clone(), from, to)
}
