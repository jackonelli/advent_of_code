#![feature(test)]
#![feature(split_inclusive)]
use std::fs::File;
use std::io::prelude::*;

//Too high: 120387
fn main() {
    let file = "input/12/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data: Vec<Instr> = contents
        .trim()
        .lines()
        .map(|line| Instr::from(line))
        .collect();
    println!("Star 1: {:?}", star_1(&data));
    println!("Star 2: {:?}", star_2(&data));
}

fn star_1(instrs: &[Instr]) -> i32 {
    let (x, y) = move_(instrs);
    x.abs() + y.abs()
}

fn star_2(instrs: &[Instr]) -> i32 {
    let (x, y) = move_rel(instrs);
    x.abs() + y.abs()
}

fn move_rel(instrs: &[Instr]) -> (i32, i32) {
    let (ship, _) = instrs.iter().fold(((0, 0), (10, 1)), |(ship, way), instr| {
        instr.move_rel(ship, way)
    });
    ship
}

fn move_(instrs: &[Instr]) -> (i32, i32) {
    let fin_pos = instrs
        .iter()
        .fold((0, 0, Dir::East), |acc, instr| instr.move_(acc));
    (fin_pos.0, fin_pos.1)
}

#[derive(Debug)]
enum Instr {
    Dir(Dir, i32),
    Rot(i32),
    Mov(i32),
}

fn rotate(way: (i32, i32), deg: i32) -> (i32, i32) {
    let (w_x, w_y) = (way.0 as f64, way.1 as f64);
    let deg = deg % 360;
    let deg = if deg < 0 { 360 + deg } else { deg };
    let rad = deg as f64 * std::f64::consts::PI / 180.0;
    let cos = rad.cos();
    let sin = rad.sin();
    let rot_x = w_x * cos - w_y * sin;
    let rot_y = w_x * sin + w_y * cos;
    (rot_x.round() as i32, rot_y.round() as i32)
}

impl Instr {
    fn move_rel(&self, ship: (i32, i32), way: (i32, i32)) -> ((i32, i32), (i32, i32)) {
        match &self {
            Instr::Dir(dir, step) => {
                let (w_x, w_y) = way;
                match dir {
                    Dir::North => (ship, (w_x, w_y + step)),
                    Dir::East => (ship, (w_x + step, w_y)),
                    Dir::South => (ship, (w_x, w_y - step)),
                    Dir::West => (ship, (w_x - step, w_y)),
                }
            }
            Instr::Rot(deg) => (ship, rotate(way, -*deg)),
            Instr::Mov(step) => {
                let (s_x, s_y) = ship;
                let (w_x, w_y) = way;
                ((s_x + step * w_x, s_y + step * w_y), way)
            }
        }
    }
    fn move_(&self, pos_head: (i32, i32, Dir)) -> (i32, i32, Dir) {
        let (x, y, head) = pos_head;
        match &self {
            Instr::Dir(dir, step) => match dir {
                Dir::North => (x, y + step, head),
                Dir::East => (x + step, y, head),
                Dir::South => (x, y - step, head),
                Dir::West => (x - step, y, head),
            },
            Instr::Rot(deg) => (x, y, head.rotate(*deg)),
            Instr::Mov(step) => match head {
                Dir::North => (x, y + step, head),
                Dir::East => (x + step, y, head),
                Dir::South => (x, y - step, head),
                Dir::West => (x - step, y, head),
            },
        }
    }

    fn from(line: &str) -> Instr {
        let (letter, qty) = line.split_at(1);
        let letter = letter.chars().next().expect("letter parse");
        let qty = qty.parse::<i32>().expect("qty parse");
        match letter {
            'N' | 'E' | 'S' | 'W' => Instr::Dir(Dir::from_instr(letter), qty),
            'R' => Instr::Rot(qty),
            'L' => Instr::Rot(-qty),
            'F' => Instr::Mov(qty),
            _ => panic!("Instr match"),
        }
    }
}

#[derive(Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn from_instr(dir: char) -> Dir {
        match dir {
            'N' => Dir::North,
            'E' => Dir::East,
            'S' => Dir::South,
            'W' => Dir::West,
            _ => panic!("Dir::match error"),
        }
    }
    fn rotate(self, deg: i32) -> Dir {
        let deg = (self.deg() + deg) % 360;
        let deg = if deg < 0 { 360 + deg } else { deg };
        match deg {
            0 => Dir::North,
            90 => Dir::East,
            180 => Dir::South,
            270 => Dir::West,
            _ => panic!(format!("bad deg {}", deg)),
        }
    }

    fn deg(&self) -> i32 {
        match self {
            Dir::North => 0,
            Dir::East => 90,
            Dir::South => 180,
            Dir::West => 270,
        }
    }
}
