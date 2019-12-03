use std::cmp;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
enum Step {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Step {
    fn from_string(step: &str) -> Self {
        let (direction, step_size) = first_char_split(step);
        let step_size: i32 = step_size.parse().expect("Parse as i32 error");
        match direction {
            "U" => Step::Up(step_size),
            "D" => Step::Down(step_size),
            "L" => Step::Left(step_size),
            "R" => Step::Right(step_size),
            _ => panic!("Invalid step direction"),
        }
    }
}

fn first_char_split(s: &str) -> (&str, &str) {
    for i in 1..5 {
        let r = s.get(0..i);
        match r {
            Some(x) => return (x, &s[i..]),
            None => (),
        }
    }

    (&s[0..0], s)
}

#[derive(Debug)]
struct HorizontalLine {
    y: i32,
    min_x: i32,
    max_x: i32,
}

impl HorizontalLine {
    fn line_intersection(&self, vert_line: &VerticalLine) -> Option<Coordinate> {
        if vert_line.x >= self.min_x
            && vert_line.x <= self.max_x
            && self.y <= vert_line.max_y
            && self.y >= vert_line.min_y
        {
            Some(Coordinate {
                x: vert_line.x,
                y: self.y,
            })
        } else {
            None
        }
    }

    fn coord_intersection<'a>(&self, coord: &'a Coordinate) -> Option<&'a Coordinate> {
        if self.y == coord.y && coord.x >= self.min_x && coord.x <= self.max_x {
            Some(coord)
        } else {
            None
        }
    }

    fn len(&self) -> u32 {
        (self.max_x - self.min_x) as u32
    }
}

#[derive(Debug)]
struct VerticalLine {
    x: i32,
    min_y: i32,
    max_y: i32,
}

impl VerticalLine {
    fn line_intersection(&self, horiz_line: &HorizontalLine) -> Option<Coordinate> {
        if self.x >= horiz_line.min_x
            && self.x <= horiz_line.max_x
            && horiz_line.y <= self.max_y
            && horiz_line.y >= self.min_y
        {
            Some(Coordinate {
                x: self.x,
                y: horiz_line.y,
            })
        } else {
            None
        }
    }

    fn coord_intersection<'a>(&self, coord: &'a Coordinate) -> Option<&'a Coordinate> {
        if self.x == coord.x && coord.y >= self.min_y && coord.y <= self.max_y {
            Some(coord)
        } else {
            None
        }
    }

    fn len(&self) -> u32 {
        (self.max_y - self.min_y) as u32
    }
}

const ORIGIN: Coordinate = Coordinate { x: 0, y: 0 };

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}
impl Coordinate {
    fn manhattan_distance(&self, reference: &Coordinate) -> u32 {
        let x_dist = (self.x - reference.x).abs();
        let y_dist = (self.y - reference.y).abs();
        (x_dist + y_dist) as u32
    }
}

impl cmp::Ord for Coordinate {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.manhattan_distance(&ORIGIN)
            .cmp(&other.manhattan_distance(&ORIGIN))
    }
}

impl cmp::Eq for Coordinate {}

#[derive(Debug)]
struct Path {
    horizontals: Vec<HorizontalLine>,
    verticals: Vec<VerticalLine>,
    step_seq: Vec<Step>,
}

impl Path {
    fn empty() -> Self {
        Path {
            horizontals: Vec::new(),
            verticals: Vec::new(),
            step_seq: Vec::new(),
        }
    }
    fn from_path_string(path: &str) -> Self {
        let steps = path.split(',').map(|x| Step::from_string(x));
        let mut path = Path::empty();
        steps.fold(ORIGIN.clone(), |current_position, step| {
            add_line_and_translate_position(step, &current_position, &mut path)
        });
        path
    }
}

fn add_line_and_translate_position(
    step: Step,
    current_position: &Coordinate,
    path: &mut Path,
) -> Coordinate {
    let new_position = match step {
        Step::Up(step_size) => {
            let line = VerticalLine {
                x: current_position.x,
                min_y: current_position.y,
                max_y: current_position.y + step_size,
            };
            path.verticals.push(line);
            Coordinate {
                x: current_position.x,
                y: current_position.y + step_size,
            }
        }
        Step::Down(step_size) => {
            let line = VerticalLine {
                x: current_position.x,
                min_y: current_position.y - step_size,
                max_y: current_position.y,
            };
            path.verticals.push(line);
            Coordinate {
                x: current_position.x,
                y: current_position.y - step_size,
            }
        }
        Step::Left(step_size) => {
            let line = HorizontalLine {
                y: current_position.y,
                min_x: current_position.x - step_size,
                max_x: current_position.x,
            };
            path.horizontals.push(line);
            Coordinate {
                x: current_position.x - step_size,
                y: current_position.y,
            }
        }
        Step::Right(step_size) => {
            let line = HorizontalLine {
                y: current_position.y,
                min_x: current_position.x,
                max_x: current_position.x + step_size,
            };
            path.horizontals.push(line);
            Coordinate {
                x: current_position.x + step_size,
                y: current_position.y,
            }
        }
    };
    path.step_seq.push(step);
    new_position
}

fn find_intersections(path_1: &Path, path_2: &Path) -> Vec<Coordinate> {
    let horizontal_intersections = path_1.horizontals.iter().flat_map(|x| {
        path_2
            .verticals
            .iter()
            .filter_map(move |y| x.line_intersection(y))
    });

    let vertical_intersections = path_1.verticals.iter().flat_map(|x| {
        path_2
            .horizontals
            .iter()
            .filter_map(move |y| x.line_intersection(y))
    });

    let intersections = horizontal_intersections.chain(vertical_intersections);

    intersections
        .filter(|x| x.manhattan_distance(&ORIGIN) > 0)
        .collect()
}

fn shortest_combined_paths(path_1: &Path, path_2: &Path, intersections: &[Coordinate]) -> u32 {
    intersections
        .iter()
        .map(|x| shortest_path(path_1, x) + shortest_path(path_2, x))
        .min()
        .expect("Could not find min")
}

fn shortest_path(path: &Path, intersection: &Coordinate) -> u32 {
    let mut horiz_counter: usize = 0;
    let mut vert_counter: usize = 0;
    let mut dist: u32 = 0;
    for step in &path.step_seq {
        match step {
            Step::Up(_) => {
                let line = &path.verticals[vert_counter];
                vert_counter += 1;
                if let Some(coord) = line.coord_intersection(intersection) {
                    dist += (coord.y - line.min_y) as u32;
                    return dist;
                } else {
                    dist += line.len();
                }
            }
            Step::Down(_) => {
                let line = &path.verticals[vert_counter];
                vert_counter += 1;
                if let Some(coord) = line.coord_intersection(intersection) {
                    dist += (line.max_y - coord.y) as u32;
                    return dist;
                } else {
                    dist += line.len();
                }
            }
            Step::Left(_) => {
                let line = &path.horizontals[horiz_counter];
                horiz_counter += 1;
                if let Some(coord) = line.coord_intersection(intersection) {
                    dist += (line.max_x - coord.x) as u32;
                    return dist;
                } else {
                    dist += line.len();
                }
            }
            Step::Right(_) => {
                let line = &path.horizontals[horiz_counter];
                horiz_counter += 1;
                if let Some(coord) = line.coord_intersection(intersection) {
                    dist += (coord.x - line.min_x) as u32;
                    return dist;
                } else {
                    dist += line.len();
                }
            }
        }
    }
    dist
}
fn star_1(file: &str) {
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let paths: Vec<&str> = contents.lines().collect();
    let path_1 = paths[0];
    let path_1 = Path::from_path_string(path_1);

    let path_2 = paths[1];
    let path_2 = Path::from_path_string(path_2);

    let intersections = find_intersections(&path_1, &path_2);

    let min_intersection = intersections.iter().min().expect("No min dist found");
    println!(
        "Minimal Manhattan distance: {:?}",
        min_intersection.manhattan_distance(&ORIGIN)
    )
}

fn star_2(file: &str) {
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let paths: Vec<&str> = contents.lines().collect();
    let path_1 = paths[0];
    let path_1 = Path::from_path_string(path_1);

    let path_2 = paths[1];
    let path_2 = Path::from_path_string(path_2);

    let intersections = find_intersections(&path_1, &path_2);
    let shortest_dist = shortest_combined_paths(&path_1, &path_2, &intersections);
    println!("Shortest dist: {}", shortest_dist)
}

fn main() {
    let file = "input/3/actual";
    star_1(file);
    star_2(file);
}

#[cfg(test)]
mod tests_3 {
    use super::*;
    #[test]
    fn test_vert_coord_intersect() {
        let line = VerticalLine {
            x: 107,
            min_y: 11,
            max_y: 78,
        };
        let coord = Coordinate { x: 107, y: 47 };
        line.coord_intersection(&coord).unwrap();
    }
}
