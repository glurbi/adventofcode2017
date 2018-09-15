use std::collections::HashSet;
use std::collections::HashMap;
use std::iter;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Node {
    id: i32,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<Node, HashSet<Node>>,
}

impl Graph {

    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn insert<'a, I>(&mut self, from_node: &Node, to_nodes: I)
    where I: Iterator<Item = &'a Node>  {
        let links = self.nodes.entry(from_node.clone()).or_insert(HashSet::new());
        for to_node in to_nodes {
            links.insert(to_node.clone());
        }
    }

    fn group(&self, node: &Node) -> HashSet<Node> {
        let mut g = HashSet::new();
        let mut visited = HashSet::new();
        self.group_r(node, &mut g, &mut visited);
        g
    }

    fn group_r(&self, node: &Node, g: &mut HashSet<Node>, visited: &mut HashSet<Node>) {
        for n in self.nodes[node].iter() {
            if visited.insert(n.clone()) {
                g.insert(n.clone());
                self.group_r(n, g, visited);
            }
        }
    }

    fn group_count(&self) -> i32 {
        let mut visited = HashSet::new();
        let mut count = 0;
        for n in self.nodes.keys() {
            if !visited.contains(n) {
                count += 1;
                let g = self.group(&n);
                visited.extend(g.into_iter());
            }
        }
        count
    }
}

pub fn day12() {
    day12_1();
    day12_2();
}

fn create_graph(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
        let mut it = line.split(" <-> ");
        let from_id = it.next().unwrap().parse::<i32>().unwrap();
        let from_node = Node { id: from_id };
        let mut to_ids = it.next().unwrap().split(", ").map(|x| x.parse::<i32>().unwrap());
        let to_nodes: HashSet<Node> = to_ids.clone().map(|id| Node { id }).collect();
        graph.insert(&from_node, to_nodes.iter());
        for to_node in to_nodes.iter() {
            graph.insert(to_node, iter::once(&from_node));
        }
    }
    graph
}

fn day12_1() {
    let input = read_to_string("input/Day12-test.txt");
    println!("{}", &input);
    let graph = create_graph(&input);
    println!("{:?}", &graph);
    let node0 = Node { id: 0};
    let group = graph.group(&node0);
    println!("{:?}", group);
    println!("{}", group.len());

    let input = read_to_string("input/Day12.txt");
    let graph = create_graph(&input);
    let group = graph.group(&node0);
    println!("{:?}", group);
    println!("{}", group.len());
}

fn day12_2() {
    let input = read_to_string("input/Day12-test.txt");
    let graph = create_graph(&input);
    println!("{}", graph.group_count());

    let input = read_to_string("input/Day12.txt");
    let graph = create_graph(&input);
    println!("{}", graph.group_count());
}

fn read_to_string(file_name: &str) -> String {
    let err_open = format!("Failed to open {}", &file_name);
    let mut file = File::open(file_name).expect(&err_open);
    let mut content = String::new();
    let err_read = format!("Failed to read {}", &file_name);
    file.read_to_string(&mut content).expect(&err_read);
    content
}