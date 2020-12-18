use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Int = u64;
fn main() {
    let file = "input/18/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data: Vec<&str> = contents.trim().lines().collect();

    let test: Vec<char> = "1 + 2 * 3 + 4 * 5 + 6".chars().collect();
    let test: Vec<char> = "1 + (2 * 3) + (4 * (5 + 6))".chars().collect();
    //let test: Vec<char> = "(2 * 3)".chars().collect();

    println!("{}", eval_expr(&test, 0, 0).0);
    println!("Star 1: {}", star_1(&data));
    println!("Star 2: {}", star_2(&data));
}

fn star_1(lines: &[&str]) -> Int {
    lines
        .iter()
        .map(|l| {
            println!("l: {}", l);
            let chars: Vec<char> = l.chars().collect();
            eval_expr(&chars, 0, 0).0
        })
        .sum()
}

fn star_2(lines: &[&str]) -> Int {
    1
}

fn eval_expr_2(expr: &[char], res: Int, idx: usize) -> (Int, usize) {
    let c = expr[idx];
    let tmp = match c {
        '(' => eval_expr(expr, 0, idx + 1),
        _ => (c.to_digit(10).expect("No first dig") as Int, idx + 1),
    };
    let mut rec_op = Op::Add;
    let mut res = tmp.0;
    let mut idx = tmp.1;
    while idx < expr.len() {
        let c = expr[idx];
        //println!("idx: {}, c: {}, res: {}", idx, c, res);
        let tmp = match c {
            '+' => {
                rec_op = Op::Add;
                (res, idx + 1)
            }
            '*' => {
                rec_op = Op::Mul;
                (res, idx + 1)
            }
            ' ' => {
                //println!("inner res: {}", res);
                (res, idx + 1)
            }
            ')' => {
                //println!(") ret res: {}", res);
                return (res, idx + 1);
            }
            '(' => {
                let tmp = eval_expr(expr, 0, idx + 1);
                (rec_op.op(res, tmp.0), tmp.1)
            }
            _ => (
                rec_op.op(res, c.to_digit(10).expect("No dig") as Int),
                idx + 1,
            ),
        };
        res = tmp.0;
        idx = tmp.1;
    }
    //println!("End ret res: {}", res);
    (res, idx + 1)
}

fn eval_expr(expr: &[char], res: Int, idx: usize) -> (Int, usize) {
    let c = expr[idx];
    let tmp = match c {
        '(' => eval_expr(expr, 0, idx + 1),
        _ => (c.to_digit(10).expect("No first dig") as Int, idx + 1),
    };
    let mut rec_op = Op::Add;
    let mut res = tmp.0;
    let mut idx = tmp.1;
    while idx < expr.len() {
        let c = expr[idx];
        //println!("idx: {}, c: {}, res: {}", idx, c, res);
        let tmp = match c {
            '+' => {
                rec_op = Op::Add;
                (res, idx + 1)
            }
            '*' => {
                rec_op = Op::Mul;
                (res, idx + 1)
            }
            ' ' => {
                //println!("inner res: {}", res);
                (res, idx + 1)
            }
            ')' => {
                //println!(") ret res: {}", res);
                return (res, idx + 1);
            }
            '(' => {
                let tmp = eval_expr(expr, 0, idx + 1);
                (rec_op.op(res, tmp.0), tmp.1)
            }
            _ => (
                rec_op.op(res, c.to_digit(10).expect("No dig") as Int),
                idx + 1,
            ),
        };
        res = tmp.0;
        idx = tmp.1;
    }
    //println!("End ret res: {}", res);
    (res, idx + 1)
}

enum Op {
    Add,
    Mul,
}

impl Op {
    fn op(&self, lhs: Int, rhs: Int) -> Int {
        match self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
        }
    }
}
#[cfg(test)]
mod test_18_2 {
    use super::*;

    #[test]
    fn test_1() {
        let test: Vec<char> = "1 + 2 * 3 + 4 * 5 + 6".chars().collect();
        assert_eq!(231, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_2() {
        let test: Vec<char> = "1 + (2 * 3) + (4 * (5 + 6))".chars().collect();
        assert_eq!(51, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_3() {
        let test: Vec<char> = "2 * 3 + (4 * 5)".chars().collect();
        assert_eq!(46, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_4() {
        let test: Vec<char> = "5 + (8 * 3 + 9 + 3 * 4 * 3)".chars().collect();
        assert_eq!(1445, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_5() {
        let test: Vec<char> = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
            .chars()
            .collect();
        assert_eq!(669060, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_6() {
        let test: Vec<char> = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            .chars()
            .collect();
        assert_eq!(23340, eval_expr(&test, 0, 0).0);
    }
}

#[cfg(test)]
mod test_18_1 {
    use super::*;

    #[test]
    fn test_1() {
        let test: Vec<char> = "1 + 2 * 3 + 4 * 5 + 6".chars().collect();
        assert_eq!(71, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_2() {
        let test: Vec<char> = "1 + (2 * 3) + (4 * (5 + 6))".chars().collect();
        assert_eq!(51, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_3() {
        let test: Vec<char> = "2 * 3 + (4 * 5)".chars().collect();
        assert_eq!(26, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_4() {
        let test: Vec<char> = "5 + (8 * 3 + 9 + 3 * 4 * 3)".chars().collect();
        assert_eq!(437, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_5() {
        let test: Vec<char> = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
            .chars()
            .collect();
        assert_eq!(12240, eval_expr(&test, 0, 0).0);
    }
    #[test]
    fn test_6() {
        let test: Vec<char> = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            .chars()
            .collect();
        assert_eq!(13632, eval_expr(&test, 0, 0).0);
    }
}
