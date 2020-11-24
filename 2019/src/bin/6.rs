use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

type NodeId = String;
type Orbits = HashMap<NodeId, Vec<NodeId>>;
type Leaves = HashMap<NodeId, u32>;
type HashTree = HashMap<NodeId, Node>;

struct Node {
    dist: u32,
    parent_id: Option<NodeId>,
}

fn parse_string_vec_to_orbit(orbit_pairs: &[&str]) -> Orbits {
    let mut orbits: Orbits = HashMap::new();
    let orbit_re = Regex::new(r"^([A-Z0-9]{1,3})\)([A-Z0-9]{1,3})$").expect("Invalid regex");
    for line in orbit_pairs {
        for cap in orbit_re.captures_iter(line) {
            let parent = NodeId::from(&cap[1]);
            let child = NodeId::from(&cap[2]);
            match orbits.get_mut(&parent) {
                Some(children) => children.push(child),
                None => {
                    orbits.insert(parent, vec![child]);
                }
            }
        }
    }
    orbits
}

fn iterate_leaves(leaves: &mut Leaves, nodes: &mut HashTree, orbits: &mut Orbits) {
    for (parent, dist) in &leaves.clone() {
        if let Some(children) = orbits.remove(&parent.clone()) {
            for child in children {
                leaves.insert(child.clone(), dist + 1);
                nodes.insert(
                    child.clone(),
                    Node {
                        dist: *dist + 1,
                        parent_id: Some(parent.clone()),
                    },
                );
            }
            leaves.remove(parent);
        }
    }
}

fn build_tree(root: &str, mut orbits: Orbits) -> HashTree {
    let mut nodes = HashMap::new();
    nodes.insert(
        root.into(),
        Node {
            dist: 0,
            parent_id: None,
        },
    );
    let mut leaves: Leaves = HashMap::new();
    leaves.insert(root.into(), 0);
    while !orbits.is_empty() {
        iterate_leaves(&mut leaves, &mut nodes, &mut orbits);
    }
    nodes
}

fn calculate_orbits(nodes: &HashTree) -> u32 {
    nodes
        .values()
        .fold(0, |total_dist, node| total_dist + node.dist)
}

fn get_parent_id(nodes: &HashTree, node_1: &str) -> NodeId {
    match nodes.get(node_1).unwrap().parent_id.clone() {
        Some(parent_id) => parent_id,
        None => NodeId::from(node_1),
    }
}

fn find_common_ancestor(nodes: &HashTree, start: &str, goal: &str) -> NodeId {
    let mut node_1 = get_parent_id(nodes, &start);
    let mut node_2 = get_parent_id(nodes, &goal);
    let mut ancestors_1 = HashSet::new();
    ancestors_1.insert(node_1.clone());
    let mut ancestors_2 = HashSet::new();
    ancestors_2.insert(node_2.clone());
    while !ancestors_1.contains(&node_2) && !ancestors_2.contains(&node_1) {
        node_1 = get_parent_id(nodes, &node_1);
        node_2 = get_parent_id(nodes, &node_2);
        ancestors_1.insert(node_1.clone());
        ancestors_2.insert(node_2.clone());
    }
    if ancestors_1.contains(&node_2) {
        node_2
    } else {
        node_1
    }
}

fn main() {
    let file = "input/6/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = NodeId::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let orbit_pairs: Vec<&str> = contents.lines().collect();
    let orbits = parse_string_vec_to_orbit(&orbit_pairs);
    let nodes = build_tree("COM", orbits);

    let num_orbits = calculate_orbits(&nodes);
    println!("Number of orbits: {}", num_orbits);

    let start = "YOU";
    let goal = "SAN";
    let common_ancestor = find_common_ancestor(&nodes, start, goal);

    let common_dist = nodes.get(&common_ancestor).unwrap().dist;
    let start_dist = nodes.get(start).unwrap().dist;
    let goal_dist = nodes.get(goal).unwrap().dist;
    println!(
        "Number of orbit jumps: {}",
        start_dist - common_dist + goal_dist - common_dist - 2
    );
}

#[cfg(test)]
mod tests_6 {
    use super::*;
    #[test]
    fn test_simple() {}
}
