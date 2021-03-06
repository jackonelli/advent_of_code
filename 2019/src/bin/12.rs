#[derive(Debug, Copy, Clone, PartialEq)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3 {
    fn abs_sum(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Point3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Point3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::SubAssign for Point3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Moon {
    pos: Point3,
    vel: Point3,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        let pos = Point3 { x, y, z };
        let vel = Point3 { x: 0, y: 0, z: 0 };
        Moon { pos, vel }
    }

    fn signed_dist(&self, other: &Moon) -> Point3 {
        Point3 {
            x: (other.pos.x - self.pos.x).signum(),
            y: (other.pos.y - self.pos.y).signum(),
            z: (other.pos.z - self.pos.z).signum(),
        }
    }

    fn energy(&self) -> i32 {
        let pot_energy = self.pos.abs_sum();
        let kin_energy = self.vel.abs_sum();
        pot_energy * kin_energy
    }
}

impl std::fmt::Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}

fn one_timestep(moons: Vec<Moon>) -> Vec<Moon> {
    let num_moons = moons.len();

    // How make functional w/ cross index?
    let mut delta_vel = vec![Point3 { x: 0, y: 0, z: 0 }; moons.len()];
    for (first, moon_1) in moons.iter().enumerate() {
        for second in (first + 1)..num_moons {
            let moon_2 = &moons[second];
            let dist = moon_1.signed_dist(moon_2);
            delta_vel[first] += dist;
            delta_vel[second] -= dist;
        }
    }
    moons
        .iter()
        .enumerate()
        .map(|moon| {
            let new_vel = moon.1.vel + delta_vel[moon.0];
            let new_pos = moon.1.pos + new_vel;
            Moon {
                pos: new_pos,
                vel: new_vel,
            }
        })
        .collect()
}

fn star_1() {
    let io: Moon = Moon::new(-19, -4, 2);
    let europa: Moon = Moon::new(-9, 8, -16);
    let ganymede: Moon = Moon::new(-4, 5, -11);
    let callisto: Moon = Moon::new(1, 9, -13);
    let mut moons = vec![io, europa, ganymede, callisto];
    for _ in 0..1000 {
        moons = one_timestep(moons);
    }
    let energy = moons.iter().fold(0, |acc, moon| acc + moon.energy());
    println!("System energy: {}", energy);
}

fn star_2() {
    let io: Moon = Moon::new(-19, -4, 2);
    let europa: Moon = Moon::new(-9, 8, -16);
    let ganymede: Moon = Moon::new(-4, 5, -11);
    let callisto: Moon = Moon::new(1, 9, -13);
    let mut moons = vec![io, europa, ganymede, callisto];
    let initial_state = moons.clone();
    let mut periods = vec![0, 0, 0];
    let mut count: u64 = 0;
    // EPA do-while!!!
    while periods.iter().any(|period| *period == 0) {
        moons = one_timestep(moons);
        count += 1;

        if moons
            .iter()
            .zip(initial_state.iter())
            .all(|(current, init)| current.pos.x == init.pos.x && current.vel.x == init.vel.x)
            && periods[0] == 0
        {
            periods[0] = count;
        }
        if moons
            .iter()
            .zip(initial_state.iter())
            .all(|(current, init)| current.pos.y == init.pos.y && current.vel.y == init.vel.y)
            && periods[1] == 0
        {
            periods[1] = count;
        }
        if moons
            .iter()
            .zip(initial_state.iter())
            .all(|(current, init)| current.pos.z == init.pos.z && current.vel.z == init.vel.z)
            && periods[2] == 0
        {
            periods[2] = count;
        }
    }
    println!("Repeats after: {:?}", least_common_multiple_n(&periods));
}

fn least_common_multiple_n(numbers: &[u64]) -> u64 {
    let mut lcm = least_common_multiple(numbers[0], numbers[1]);
    for number in numbers.into_iter().skip(2) {
        lcm = least_common_multiple(lcm, *number);
    }
    lcm
}

fn least_common_multiple(num_1: u64, num_2: u64) -> u64 {
    let (mut numerator, mut denominator) = if num_1 > num_2 {
        (num_1, num_2)
    } else {
        (num_2, num_1)
    };
    let mut remainder = numerator % denominator;
    while remainder != 0 {
        numerator = denominator;
        denominator = remainder;
        remainder = numerator % denominator;
    }
    let greatest_common_divisor = denominator;
    num_1 * num_2 / greatest_common_divisor
}

fn main() {
    env_logger::init();
    // star_1();
    star_2();
}

#[cfg(test)]
mod tests_12 {
    use super::*;
    #[test]
    fn test_10() {
        let io: Moon = Moon::new(-1, 0, 2);
        let europa: Moon = Moon::new(2, -10, -7);
        let ganymede: Moon = Moon::new(4, -8, 8);
        let callisto: Moon = Moon::new(3, 5, -1);
        let mut moons = vec![io, europa, ganymede, callisto];
        for _ in 0..10 {
            moons = one_timestep(moons);
        }
        let energy = moons.iter().fold(0, |acc, moon| acc + moon.energy());
        assert_eq!(energy, 179);
        assert_eq!(moons[0].pos, Point3 { x: 2, y: 1, z: -3 });
    }
    #[test]
    fn test_100() {
        let io: Moon = Moon::new(-8, -10, 0);
        let europa: Moon = Moon::new(5, 5, 10);
        let ganymede: Moon = Moon::new(2, -7, 3);
        let callisto: Moon = Moon::new(9, -8, -3);
        let mut moons = vec![io, europa, ganymede, callisto];
        for _ in 0..100 {
            moons = one_timestep(moons);
        }
        let energy = moons.iter().fold(0, |acc, moon| acc + moon.energy());
        assert_eq!(energy, 1940);
        assert_eq!(moons[2].vel, Point3 { x: -3, y: 7, z: 4 });
    }
    #[test]
    fn test_recurring_state() {
        let io: Moon = Moon::new(-1, 0, 2);
        let europa: Moon = Moon::new(2, -10, -7);
        let ganymede: Moon = Moon::new(4, -8, 8);
        let callisto: Moon = Moon::new(3, 5, -1);
        let mut moons = vec![io, europa, ganymede, callisto];
        let initial_state = moons.clone();
        let mut count: u64 = 0;
        // EPA do-while!!!
        while {
            moons = one_timestep(moons);
            count += 1;
            moons
                .iter()
                .zip(initial_state.iter())
                .any(|pair| pair.0 != pair.1)
        } {}
        assert_eq!(count, 2772);
    }
    #[test]
    fn test_lcm() {
        let numbers = vec![161428, 113028, 231614];
        assert_eq!(528250271633772, least_common_multiple_n(&numbers));
    }
    #[test]
    fn test_putnik() {
        let numbers = vec![231613, 96235, 193051];
        assert_eq!(4302967224744805, least_common_multiple_n(&numbers));
    }
}
