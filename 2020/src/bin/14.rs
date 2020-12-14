use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Addr = u64;
type Val = u64;
type BitVal = u8;
type BinVal = Vec<BitVal>;
type BitIdx = usize;
type MaskNonX = Vec<(BitIdx, BitVal)>;
type Mask = Vec<Bit>;
type Mem = HashMap<Addr, Val>;

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
        if l.contains("mask") {
            mask = get_mask(l);
        } else {
            let (addr, val) = get_mem_val(l);
            let masked = apply_mask(val, &mask);
            mem.insert(addr, masked);
        }
    }
    mem.iter().map(|(_, val)| *val).sum()
}

fn star_2(lines: &[&str]) -> Val {
    let mut mem = Mem::new();
    let mut mask_perms = Vec::new();
    for l in lines {
        if l.contains("mask") {
            let mask = get_mask(l);
            mask_perms = perms(&mask, Vec::new());
        } else {
            let (addr, val) = get_mem_val(l);
            for mask in &mask_perms {
                let masked = apply_addr_mask(addr, mask);
                mem.insert(masked, val);
            }
        }
    }
    mem.iter().map(|(_, val)| *val).sum()
}

fn perms(mask: &Mask, masks: Vec<Mask>) -> Vec<Mask> {
    if mask.iter().filter(|b| **b == Bit::Fix('X')).count() == 0 {
        let mut new_masks = masks;
        new_masks.push(mask.clone());
        new_masks
    } else {
        let x_idx = mask.iter().position(|b| *b == Bit::Fix('X')).unwrap();
        let mut new_mask = mask.to_vec();
        new_mask[x_idx] = Bit::Float('0');
        let zero_tree = perms(&new_mask, masks.clone());
        new_mask[x_idx] = Bit::Float('1');
        let mut one_tree = perms(&new_mask, masks);
        // Join the two branches.
        one_tree.extend_from_slice(&zero_tree);
        one_tree
    }
}

fn apply_addr_mask(addr: Addr, mask: &Mask) -> Val {
    let mut bin = dec_to_bin(addr);
    for (idx, bit) in mask.iter().enumerate() {
        match bit {
            Bit::Fix(c) => match c {
                '0' => {}
                '1' => bin[idx] = 1,
                _ => panic!("Unparsed bit"),
            },
            Bit::Float(c) => match c {
                '0' => bin[idx] = 0,
                '1' => bin[idx] = 1,
                _ => panic!("Unparsed bit"),
            },
        }
    }
    let masked = bin_to_dec(&bin);
    masked
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Bit {
    Fix(char),
    Float(char),
}
impl Bit {
    fn get_val(&self) -> char {
        match self {
            Bit::Fix(c) => *c,
            Bit::Float(c) => *c,
        }
    }
}

fn apply_mask(val: Val, mask: &Mask) -> Val {
    let mask = mask
        .iter()
        .enumerate()
        .filter(|(_idx, c)| **c == Bit::Fix('1') || **c == Bit::Fix('0'))
        .map(|(idx, c)| (idx, c.get_val().to_digit(2).unwrap() as u8))
        .collect::<MaskNonX>();
    let mut bin = dec_to_bin(val);
    for (addr, bit) in mask {
        bin[addr] = bit;
    }
    bin_to_dec(&bin)
}

fn get_mem_val(raw: &str) -> (Addr, Val) {
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
    mask.chars().map(Bit::Fix).collect()
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

fn _print_mask(mask: &Mask) {
    for c in mask {
        let c = match c {
            Bit::Float(c) => c,
            Bit::Fix(c) => c,
        };
        print!("{}", c);
    }
    println!();
}
