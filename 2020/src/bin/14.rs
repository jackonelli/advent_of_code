use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Addr = u64;
type Val = u64;
type BitVal = u8;
type BinVal = Vec<BitVal>;
type BitIdx = usize;
type Mask = Vec<(BitIdx, BitVal)>;
type Mem = HashMap<Addr, Val>;

// Too low: 43741451956
fn main() {
    let file = "input/14/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data: Vec<&str> = contents.trim().lines().collect();

    println!("Star 1: {}", star_1(&data));
    println!("Star 2: {}", star_2(&data));
}

fn star_1(lines: &[&str]) -> Val {
    let mut mem = Mem::new();
    let mut mask = Mask::new();
    for l in lines {
        println!("{}", l);
        if l.contains("mask") {
            mask = get_mask(l);
        } else {
            let (addr, val) = get_mem_val(l);
            //println!("{:?}", (addr, val));
            let masked = apply_mask(val, &mask);
            println!("Addr, Dec: {:?}, {:?}", addr, masked);
            mem.insert(addr, masked);
        }
        println!("***");
    }
    mem.iter().map(|(_, val)| *val).sum()
}

fn star_2(lines: &[&str]) -> usize {
    1
}

fn apply_mask(val: Val, mask: &Mask) -> Val {
    let mut bin = dec_to_bin(val);
    for (addr, bit) in mask {
        bin[*addr] = *bit;
    }
    let masked = bin_to_dec(&bin);
    masked
}

fn get_mem_val(raw: &str) -> (Addr, Val) {
    println!("raw: {}", raw);
    let re = regex::Regex::new(r"mem\[(\d+)\]").unwrap();
    let mut some = raw.split(" = ");
    let addr = some.next().unwrap();
    let addr = re
        .captures(addr)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<Addr>()
        .unwrap();
    let val = some.next().unwrap().parse::<Val>().unwrap();
    (addr as Addr, val)
}

fn get_mask(line: &str) -> Mask {
    let mut mask = line.split(" = ");
    mask.next().unwrap();
    let mask = mask.next().unwrap();
    mask.chars()
        //.rev()
        .enumerate()
        .filter(|(idx, c)| *c == '1' || *c == '0')
        .map(|(idx, c)| (idx, c.to_digit(10).unwrap() as u8))
        .collect::<Mask>()
}

fn bin_to_dec(bin: &BinVal) -> Val {
    bin.iter().fold(0, |x, b| x * 2 + *b as u64)
}

fn dec_to_bin(mut decimal: Val) -> BinVal {
    if decimal == 0 {
        vec![0; 36]
    } else {
        let mut bits = BinVal::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push(0);
            } else {
                bits.push(1);
            }
            decimal /= 2;
        }
        bits = bits.into_iter().rev().collect();
        if bits.len() == 36 {
            bits
        } else {
            let mut zero_pad = vec![0; 36 - bits.len()];
            zero_pad.extend(&bits);
            zero_pad
        }
    }
}

//fn binary(bin: &str, cmp: char) -> u32 {
//    bin.chars()
//        .map(|c| (c == cmp) as u32)
//        .fold(0, |x, b| x * 2 + b)
//}
