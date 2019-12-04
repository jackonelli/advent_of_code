const START: u32 = 125_730;
const END: u32 = 579_381;

fn validate_password(password: u32, validators: &[&dyn Fn(&[u8]) -> bool]) -> bool {
    let password = into_vec(password);
    validators.iter().map(|f| f(&password)).all(|x| x)
}

fn into_vec(number: u32) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = number;
    while n > 9 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.push(n as u8);
    digits.reverse();
    digits
}

fn monotonic(password: &[u8]) -> bool {
    let prev = password.iter();
    let current = password.iter().skip(1);
    prev.zip(current).map(|x| x.0 <= x.1).all(|x| x)
}

fn two_consequtive(password: &[u8]) -> bool {
    let prev = password.iter();
    let current = password.iter().skip(1);
    prev.zip(current).map(|x| x.0 == x.1).any(|x| x)
}

fn only_two_consequtive(password: &[u8]) -> bool {
    let mut index = 1;
    while index < password.len() {
        if password[index - 1] == password[index] {
            let current_common = password[index];
            let mut internal_counter = 1;
            while index < password.len() - 1 && password[index + 1] == current_common {
                internal_counter += 1;
                index += 1;
            }
            if internal_counter == 1 {
                return true;
            }
        }
        index += 1;
    }
    false
}

fn _rec_top(password: &[u8]) -> bool {
    _rec(password, 1, password[0], false)
}

fn _rec(password: &[u8], current_index: usize, prev_value: u8, candidate: bool) -> bool {
    if current_index == password.len() - 1 {
        return false;
    }
    if password[current_index] == prev_value {
        if candidate {
            _rec(password, current_index + 1, prev_value, false)
        } else {
            _rec(password, current_index + 1, prev_value, true)
        }
    } else {
        if candidate {
            true
        } else {
            _rec(password, current_index + 1, prev_value, false)
        }
    }
}

fn galaxy_brain(start: u32, end: u32) -> (u32, u32) {
    let mut valid_count = 0;
    let mut checked_numbers = 0;
    for first in (start / 100_000)..10 {
        for second in first..10 {
            for third in second..10 {
                for fourth in third..10 {
                    for fifth in fourth..10 {
                        for sixth in fifth..10 {
                            let number: u32 = 100_000 * first
                                + 10_000 * second
                                + 1000 * third
                                + 100 * fourth
                                + 10 * fifth
                                + sixth;
                            if number < start {
                                continue;
                            }
                            if number > end {
                                return (valid_count, checked_numbers);
                            }
                            let number_vec: &[u8] = &[
                                first as u8,
                                second as u8,
                                third as u8,
                                fourth as u8,
                                fifth as u8,
                                sixth as u8,
                            ];
                            checked_numbers += 1;
                            if two_consequtive(&number_vec) {
                                valid_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    (valid_count, checked_numbers)
}

fn main() {
    let (_count, checked) = galaxy_brain(START, END);
    println!(
        "Galaxy brain, checked {}/{}, {}%",
        checked,
        END - START,
        checked as f32 / (END - START) as f32
    );
    let range = START..END;
    let validators: &[&dyn Fn(&[u8]) -> bool] = &[&monotonic, &two_consequtive];
    let valid_passwords = range.filter(|x| validate_password(*x, validators));
    println!(
        "Number of valid passwords: {}",
        valid_passwords.clone().count()
    );
    let validators: &[&dyn Fn(&[u8]) -> bool] = &[&only_two_consequtive];
    let valid_passwords = valid_passwords.filter(|x| validate_password(*x, validators));
    println!(
        "Number of only double valid passwords: {}",
        valid_passwords.count()
    );
}

#[cfg(test)]
mod tests_4 {
    use super::*;
    #[test]
    fn test_only() {
        let number = 112233;
        //let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&monotonic, &only_two_consequtive];
        let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&_rec_top];
        assert!(validate_password(number, &validators));
    }
    #[test]
    fn test_only2() {
        let number = 123444;
        //let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&monotonic, &only_two_consequtive];
        let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&_rec_top];
        assert!(!validate_password(number, &validators));
    }
    #[test]
    fn test_only3() {
        let number = 111122;
        //let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&monotonic, &only_two_consequtive];
        let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&_rec_top];
        assert!(validate_password(number, &validators));
    }
}
