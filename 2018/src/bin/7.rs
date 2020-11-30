use itertools::Itertools;
use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;
use petgraph::Direction;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;
use std::iter::FromIterator;

type Idx = u32;
type Graph = StableDiGraph<(), ()>;
type IMap = HashMap<Idx, char>;
type RevIMap = HashMap<char, Idx>;

fn main() {
    let file = "input/7/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let input = contents;
    let (graph, i_map) = parse_graph(input.lines());
    let order = gen_order(graph, i_map);
    let string = String::from_iter(order.iter());
    println!("{}", string);
}

fn gen_order(graph: Graph, i_map: IMap) -> Vec<char> {
    let mut graph = graph;
    let mut order = Vec::new();
    while graph.node_count() > 0 {
        let mut root_nodes: Vec<(Idx, char)> = root_nodes(&graph)
            .map(|idx| (idx, *i_map.get(&idx).expect("No idx (order)")))
            .collect();
        root_nodes.sort_unstable_by_key(|v| v.1);
        let next_in_line = root_nodes[0];
        graph.remove_node(NodeIndex::new(next_in_line.0 as usize));
        order.push(next_in_line.1)
    }
    order
}

fn root_nodes(graph: &Graph) -> impl Iterator<Item = Idx> + '_ {
    graph
        .node_indices()
        .filter(move |v| graph.neighbors_directed(*v, Direction::Incoming).count() == 0)
        .map(|v| v.index() as Idx)
}

fn parse_graph(instructions: std::str::Lines) -> (Graph, IMap) {
    let reg = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")
        .expect("Invalid regex");
    let edges: Vec<(char, char)> = instructions
        .map(|x| {
            let mut src = None;
            let mut dst = None;
            for cap in reg.captures_iter(x) {
                src = Some(String::from(&cap[1]));
                dst = Some(String::from(&cap[2]));
            }
            (src, dst)
        })
        .filter(|x| x.0.is_some() && x.1.is_some())
        .map(|x| (x.0.expect("src error"), x.1.expect("dst error")))
        .map(|x| {
            (
                x.0.chars().next().expect("char conv error"),
                x.1.chars().next().expect("char conv error"),
            )
        })
        .collect();
    let i_map: IMap = edges
        .iter()
        .flat_map(|x| once(x.0).chain(once(x.1)))
        .unique()
        .enumerate()
        .map(|(idx, c)| (idx as Idx, c))
        .collect();
    let rev_i_map: RevIMap = i_map.clone().into_iter().map(|x| (x.1, x.0)).collect();
    let int_edges: Vec<(Idx, Idx)> = edges
        .iter()
        .map(|(src, dst)| {
            (
                *rev_i_map.get(src).expect("rev map no src"),
                *rev_i_map.get(dst).expect("rev map no dst"),
            )
        })
        .collect();
    (Graph::from_edges(&int_edges), i_map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {}
}
