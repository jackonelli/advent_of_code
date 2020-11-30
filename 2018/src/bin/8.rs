use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Tree {
    meta_data: Vec<usize>,
    sub_trees: Vec<Tree>,
}

impl Tree {
    fn from_lic(idx: usize, lic: &[usize]) -> (usize, Tree) {
        let mut idx = idx;
        let num_children = lic[idx];
        let num_metadata = lic[idx + 1];
        idx += 2;
        let mut sub_trees: Vec<Tree> = Vec::new();
        for _ in 0..num_children {
            let (tmp, sub_tree) = Tree::from_lic(idx, lic);
            idx = tmp;
            sub_trees.push(sub_tree);
        }
        let end_meta = idx + num_metadata;
        let meta_data = (&lic[idx..end_meta]).to_vec();
        let tree = Tree {
            meta_data,
            sub_trees,
        };
        (end_meta, tree)
    }

    fn sum_metadata(&self) -> usize {
        self.meta_data.iter().sum::<usize>()
            + self
                .sub_trees
                .iter()
                .map(|t| t.sum_metadata())
                .sum::<usize>()
    }

    fn cond_sum(&self) -> usize {
        if self.sub_trees.is_empty() {
            self.meta_data.iter().sum()
        } else {
            self.meta_data
                .iter()
                .filter_map(|x| {
                    if *x > self.sub_trees.len() {
                        None
                    } else {
                        Some(self.sub_trees[x - 1].cond_sum())
                    }
                })
                .sum()
        }
    }
}

fn main() {
    let file = "input/8/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data: Vec<usize> = contents
        .trim()
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let (_, tree) = Tree::from_lic(0, &data);
    println!("Star 1: {}", tree.sum_metadata());
    println!("Star 2: {}", tree.cond_sum());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {}
}
