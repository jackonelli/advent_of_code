use aoc_2019::read_csv;

fn calculate_fuel(weight: i32) -> i32 {
    let quotient = weight as f32 / 3.0;
    quotient.floor() as i32 - 2
}

fn recursive_fuel(weight: i32) -> i32 {
    let fuel = calculate_fuel(weight);
    if fuel <= 0 {
        0
    } else {
        fuel + recursive_fuel(fuel)
    }
}

fn main() {
    let mut data: Vec<i32> = Vec::new();
    let mut reader = read_csv("data/1/input", Some(false)).expect("CSV error");
    for result in reader.deserialize() {
        let record: i32 = result.expect("Read line error");
        data.push(record);
    }

    // Functional from here:
    // Make immutable after populating it.

    let module_weights = data;

    let total_fuel: i32 = module_weights.iter().map(|x| calculate_fuel(*x)).sum();
    println!("Total fuel required: {} masses", total_fuel);

    let total_fuel_recursive: i32 = module_weights.iter().map(|x| recursive_fuel(*x)).sum();
    println!(
        "Total recursive fuel required: {} masses",
        total_fuel_recursive
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_vector_map() {
        let weights_vec = vec![12, 14, 1969, 100756];
        let true_fuel_vec = vec![2, 2, 654, 33583];
        let module_fuel: Vec<i32> = weights_vec.into_iter().map(|x| calculate_fuel(x)).collect();
        assert_eq!(module_fuel, true_fuel_vec);
    }

    #[test]
    fn test_total_sum() {
        let weights_vec = vec![12, 14, 1969, 100756];
        let total_fuel: i32 = weights_vec.into_iter().map(|x| calculate_fuel(x)).sum();
        assert_eq!(total_fuel, 34241);
    }

    #[test]
    fn test_recursive() {
        assert_eq!(recursive_fuel(14), 2);
        assert_eq!(recursive_fuel(1969), 966);
        assert_eq!(recursive_fuel(100756), 50346);
    }
}
