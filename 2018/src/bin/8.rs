use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Tree {
    meta_data: Vec<usize>,
    sub_trees: Vec<Box<Tree>>,
}

impl Tree {
    fn from_lic(idx: usize, lic: &[usize]) -> (usize, Tree) {
        let mut idx = idx;
        let num_children = lic[idx];
        let num_metadata = lic[idx + 1];
        idx += 2;
        println!("{}: {}, {}", idx - 2, num_children, num_metadata);
        let mut sub_trees: Vec<Box<Tree>> = Vec::new();
        for _ in 0..num_children {
            println!("Idx in loop: {}", idx);
            let (tmp, sub_tree) = Tree::from_lic(idx, lic);
            println!("Next idx in loop: {}", tmp);
            idx = tmp;
            sub_trees.push(Box::new(sub_tree));
        }
        let end_meta = idx + num_metadata;
        let meta_data = (&lic[idx..end_meta]).to_vec();
        let tree = Tree {
            meta_data,
            sub_trees,
        };
        println!("{}: {:?}", end_meta, tree);
        (end_meta, tree)
    }
}

fn main() {
    let file = "input/8/test";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data: Vec<usize> = contents
        .trim()
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    println!("{:?}", data);
    println!("{:?}", Tree::from_lic(0, &data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {}
}
