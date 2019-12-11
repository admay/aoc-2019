use std::fmt::{self, Display};

const LAYER_WIDTH: usize = 25;
const LAYER_HEIGHT: usize = 6;

#[derive(Clone)]
struct Layer {
    data: Vec<Vec<char>>,
}

impl Layer {
    fn new(layer_data: &[char], width: usize) -> Self {
        let mut rows = vec![];
        for i in (0..layer_data.len()).step_by(width) {
            rows.push(layer_data[i..i + width].to_owned());
        }
        Layer { data: rows }
    }

    fn rows(&self) -> impl Iterator<Item = &Vec<char>> {
        self.data.iter()
    }

    fn rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<char>> {
        self.data.iter_mut()
    }

    fn value_at(&self, row: usize, col: usize) -> char {
        self.data[row][col]
    }

    fn count_pixels(&mut self) -> (u32, u32, u32) {
        let (mut z, mut o, mut t) = (0, 0, 0);
        for row in self.rows_mut() {
            for c in row {
                match c {
                    '0' => z += 1,
                    '1' => o += 1,
                    '2' => t += 1,
                    _ => {}
                }
            }
        }
        (z, o, t)
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            for c in row {
                match c {
                    '0' => write!(f, "{}", ' ')?,
                    '1' => write!(f, "{}", '*')?,
                    _ => {}
                }
            }
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}

struct Image {
    layers: Vec<Layer>,
}

impl Image {
    fn new(img_data: &Vec<char>, width: usize, height: usize) -> Self {
        let mut layers = vec![];
        let step = width * height;
        for i in (0..img_data.len()).step_by(step) {
            let layer = &img_data[i..(i + step)];
            layers.push(Layer::new(layer, width));
        }
        Image { layers: layers }
    }

    fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.layers.iter()
    }

    fn top_visible_pixel(&self, i: usize, j: usize) -> Option<char> {
        for layer in self.layers() {
            let pixel = layer.value_at(i, j);
            if pixel != '2' {
                return Some(pixel);
            }
        }
        None
    }

    fn flatten(&self) -> Layer {
        let mut visible_layer = self.layers().nth(0).unwrap().clone();

        for (i, row) in visible_layer.rows_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if *col == '2' {
                    *col = self.top_visible_pixel(i, j).unwrap();
                }
            }
        }
        visible_layer
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Image {
    Image::new(&input.chars().collect(), LAYER_WIDTH, LAYER_HEIGHT)
}

#[aoc(day8, part1)]
fn solve_p1(img: &Image) -> u32 {
    let mut min_zeros = std::u32::MAX;
    let mut one_times_two = 0;
    for layer in img.layers() {
        let (z, o, t) = layer.clone().count_pixels();
        if z < min_zeros {
            one_times_two = o * t;
            min_zeros = z;
        }
    }
    one_times_two
}

#[aoc(day8, part2)]
fn solve_p2(img: &Image) -> u32 {
    let visible_layer: Layer = img.flatten();
    println!("{}", visible_layer);
    0
}
