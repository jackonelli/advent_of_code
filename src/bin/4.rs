const START: u32 = 125730;
const END: u32 = 579381;

fn validate_password(password: u32, validators: &Vec<&dyn Fn(&[u8]) -> bool>) -> bool {
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
    for index in 1..password.len() {
        if password[index - 1] > password[index] {
            return false;
        }
    }
    true
}

fn two_consequtive(password: &[u8]) -> bool {
    for index in 1..password.len() {
        if password[index - 1] == password[index] {
            return true;
        }
    }
    false
}

fn two_only_consequtive(password: &[u8]) -> bool {
    let mut index = 1;
    while index < password.len() {
        //println!("{} {}", index, password[index]);
        //println!("------");
        if password[index - 1] == password[index] {
            let current_common = password[index];
            let mut internal_counter = 1;
            //println!("Match! {}", index);
            while index < password.len() - 1 && password[index + 1] == current_common {
                //println!("Skipping! {}", index);
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

fn main() {
    let range = START..END;
    let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&monotonic, &two_consequtive];
    let valid_passwords = range.filter(|x| validate_password(*x, &validators));
    println!(
        "Number of valid passwords: {}",
        valid_passwords.clone().count()
    );
    let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&two_only_consequtive];
    let valid_passwords = valid_passwords.filter(|x| validate_password(*x, &validators));
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
        let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&monotonic, &two_only_consequtive];
        assert!(validate_password(number, &validators));
    }
    #[test]
    fn test_only2() {
        let number = 123444;
        let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&monotonic, &two_only_consequtive];
        assert!(!validate_password(number, &validators));
    }
    #[test]
    fn test_only3() {
        let number = 111122;
        let validators: Vec<&dyn Fn(&[u8]) -> bool> = vec![&monotonic, &two_only_consequtive];
        assert!(validate_password(number, &validators));
    }
}
