use env_logger;
use log::{debug, info};
use std::collections::{BTreeMap, HashSet};
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

    fn default() -> Self {
        Asteroid { x: -1, y: -1 }
    }

    fn dist_and_angle_to(&self, other: &Asteroid) -> (f32, f32) {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
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
    let mut most_visible = Asteroid::default();
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
) -> BTreeMap<i32, Vec<(Asteroid, f32)>> {
    let mut dist_and_angle: BTreeMap<i32, Vec<(Asteroid, f32)>> = BTreeMap::new();
    for asteroid in asteroids {
        if asteroid == station {
            continue;
        };
        let (dist, angle) = station.dist_and_angle_to(asteroid);
        let hash = hash_angle(angle);
        let entry = dist_and_angle.entry(hash).or_insert(Vec::new());
        entry.push((asteroid.clone(), dist));
    }

    for (_, dist_vec) in dist_and_angle.iter_mut() {
        dist_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    }

    for (angle, dist_vec) in &dist_and_angle {
        debug!("{}: {:?}", *angle as f32 * 0.18 / PI, dist_vec);
    }
    dist_and_angle
}

fn find_nth_blasted(station: &Asteroid, asteroids: &[Asteroid], number: usize) -> Asteroid {
    let mut angle_dist = get_sorted_map_of_angle_dist(station, asteroids);
    let mut counter = 1;
    let mut nth = Asteroid::default();
    while counter < number {
        for (_, dist_vec) in angle_dist.iter_mut() {
            if let Some(asteroid) = dist_vec.pop() {
                debug!(
                    "Blasting nr {}: ({}, {})",
                    counter, asteroid.0.x, asteroid.0.y
                );
                nth = asteroid.0;
            };
            if counter == number {
                break;
            }
            counter += 1;
        }
    }
    nth
}

fn main() {
    env_logger::init();
    let file = "input/10/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    info!("\n{}", contents);
    let asteroids = parse_asteroids(&contents);
    let (count, asteroid) = find_most_visible(&asteroids);
    println!("Visible: {}, {:?}", count, asteroid);
    let station = Asteroid::new(11, 11);
    let nth = find_nth_blasted(&station, &asteroids, 200);
    println!("{:?}", nth);
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
