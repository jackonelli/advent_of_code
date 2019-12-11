use env_logger;
use log::{debug, info};
use std::cmp;
use std::collections::{BTreeMap, BinaryHeap, HashSet};
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn new(x: i32, y: i32) -> Self {
        Asteroid { x, y }
    }

    fn placeholder() -> Self {
        Asteroid { x: -1, y: -1 }
    }

    fn dist_and_angle_to(&self, other: &Asteroid) -> (f32, f32) {
        let delta_x = other.x - self.x;
        // Invert height since they use dumb coord syst.
        let delta_y = -(other.y - self.y);
        let dist = ((delta_x as f32).powi(2) + (delta_y as f32).powi(2)).sqrt();
        let angle = (delta_y as f32 / dist).asin();
        // Project angle to correct half-plane.
        let angle = if delta_x > 0 { angle } else { PI - angle };
        // Rotate to have zero radians at positive y-axis
        let angle = PI / 2.0 - angle;
        // Translate angle to [0, 2 pi]
        let angle = if angle < 0.0 { angle + 2.0 * PI } else { angle };
        (dist, angle)
    }
}

fn hash_angle(angle: f32) -> i32 {
    (1000.0 * angle).round() as i32
}

fn parse_asteroids(contents: &str) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, char_) in line.chars().enumerate() {
            if char_ == '#' {
                asteroids.push(Asteroid::new(x as i32, y as i32))
            }
        }
    }
    asteroids
}

struct AsteroidDist {
    asteroid: Asteroid,
    dist: f32,
}

impl PartialEq for AsteroidDist {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for AsteroidDist {}

impl cmp::PartialOrd for AsteroidDist {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl cmp::Ord for AsteroidDist {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.dist
            .partial_cmp(&other.dist)
            .expect("Could not order distance")
    }
}

fn find_number_of_visible(current: &Asteroid, asteroids: &[Asteroid]) -> usize {
    let mut unique_angles = HashSet::new();
    for other in asteroids {
        if current == other {
            continue;
        };
        debug!("({}, {})", other.x, other.y);
        let (_, angle) = current.dist_and_angle_to(other);
        let deci_angle = hash_angle(angle);
        debug!("Angles: {}, {}", angle, deci_angle);
        unique_angles.insert(deci_angle);
        debug!("{}", unique_angles.len());
        debug!("---------");
    }
    unique_angles.len()
}

fn find_most_visible(asteroids: &[Asteroid]) -> (usize, Asteroid) {
    let mut most_visible = Asteroid::placeholder();
    let mut count = 0;
    for current in asteroids {
        let current_count = find_number_of_visible(current, asteroids);
        debug!("Current count: {}", current_count);
        if current_count > count {
            count = current_count;
            most_visible = current.clone();
        }
    }
    (count, most_visible)
}

fn get_sorted_map_of_angle_dist(
    station: &Asteroid,
    asteroids: &[Asteroid],
) -> BTreeMap<i32, BinaryHeap<AsteroidDist>> {
    let dist_and_angle: BTreeMap<i32, BinaryHeap<AsteroidDist>> = asteroids
        .iter()
        .filter(|asteroid| *asteroid != station)
        .fold(BTreeMap::new(), |mut map, asteroid| {
            let (dist, angle) = station.dist_and_angle_to(asteroid);
            let hash = hash_angle(angle);
            let entry = map.entry(hash).or_insert(BinaryHeap::new());
            entry.push(AsteroidDist {
                asteroid: asteroid.clone(),
                dist,
            });
            map
        });
    dist_and_angle
}

fn find_nth_blasted(station: &Asteroid, asteroids: &[Asteroid], number: usize) -> Asteroid {
    let mut angle_dist = get_sorted_map_of_angle_dist(station, asteroids);
    let mut counter = 0;
    while !angle_dist.is_empty() {
        for (_, distances) in angle_dist.iter_mut() {
            let zapped_asteroid = distances.pop().expect("Empty despite filter");
            counter += 1;
            if (counter) == number {
                return zapped_asteroid.asteroid;
            }
        }
        angle_dist = angle_dist
            .into_iter()
            .filter(|(_, distances)| !distances.is_empty())
            .collect()
    }
    panic!("Could not zap {} asteroids, found only {}", number, counter);
}

fn star_1() {
    let file = "input/10/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    info!("\n{}", contents);
    let asteroids = parse_asteroids(&contents);
    let (count, asteroid) = find_most_visible(&asteroids);
    println!("Visible: {}, {:?}", count, asteroid);
}

fn star_2() {
    let file = "input/10/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    info!("\n{}", contents);
    let asteroids = parse_asteroids(&contents);
    let station = Asteroid::new(11, 11);
    let nth = find_nth_blasted(&station, &asteroids, 200);
    println!("Last zipped {:?}", nth);
}

fn main() {
    env_logger::init();
    star_1();
    star_2();
}

#[cfg(test)]
mod tests_10 {
    use super::*;
    #[test]
    fn test_1() {
        let file = "input/10/3_4_8";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let asteroids = parse_asteroids(&contents);
        let (count, asteroid) = find_most_visible(&asteroids);
        assert_eq!(count, 8);
        assert_eq!(asteroid.x, 3);
        assert_eq!(asteroid.y, 4);
    }
    #[test]
    fn test_2() {
        let file = "input/10/5_8_33";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let asteroids = parse_asteroids(&contents);
        let (count, asteroid) = find_most_visible(&asteroids);
        assert_eq!(count, 33);
        assert_eq!(asteroid.x, 5);
        assert_eq!(asteroid.y, 8);
    }
    #[test]
    fn test_3() {
        let file = "input/10/1_2_35";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let asteroids = parse_asteroids(&contents);
        let (count, asteroid) = find_most_visible(&asteroids);
        assert_eq!(count, 35);
        assert_eq!(asteroid.x, 1);
        assert_eq!(asteroid.y, 2);
    }
    #[test]
    fn test_4() {
        let file = "input/10/6_3_41";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let asteroids = parse_asteroids(&contents);
        let (count, asteroid) = find_most_visible(&asteroids);
        assert_eq!(count, 41);
        assert_eq!(asteroid.x, 6);
        assert_eq!(asteroid.y, 3);
    }
    #[test]
    fn test_5() {
        let file = "input/10/11_13_210";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let asteroids = parse_asteroids(&contents);
        let (count, asteroid) = find_most_visible(&asteroids);
        assert_eq!(count, 210);
        assert_eq!(asteroid.x, 11);
        assert_eq!(asteroid.y, 13);
    }
}
