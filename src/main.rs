use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::collections::vec_deque::VecDeque;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, BufRead};
use std::{thread, time};

struct Graph {
    vertices: HashMap<u64, Vertex>,
    adjacencies: HashMap<u64, Vec<u64>>,
}

#[derive(Clone, Debug)]
struct Vertex {
    id : String,
    pressure: usize
}

struct Path {
    from: u64,
    to: u64,
    len: usize,
    pressure: usize,
    vertices :Vec<u64>
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn calculate_shortest_path(graph: &Graph, start: u64, end: u64) -> Vec<u64> {

    let mut dist: HashMap<u64, usize> = HashMap::new();
    let mut prev: HashMap<u64, u64> = HashMap::new();
    let mut q: HashSet<u64> = HashSet::new();

    for (h, _v) in graph.vertices.iter(){
        dist.insert(*h, usize::MAX);
        q.insert(*h);
    }

    dist.insert(start, 0);

    while !q.is_empty() {

        let u = *q.iter().min_by(|&h1, &h2| dist[h1].cmp(&dist[h2])).unwrap();

        if u == end {

            let mut path: Vec<u64> = Vec::new();
            let mut current = end;

            while current != start {
                path.push(current);
                current = prev[&current];
            }

            return path;
        }

        q.remove(&u);

        for neighbour in graph.adjacencies[&u].iter().filter(|h| q.contains(h)) {
            let alt = dist[&u] + 1;

            if alt < dist[neighbour] {
                dist.insert(*neighbour, alt);
                prev.insert(*neighbour, u);
            }
        }
    }

    return vec![];
}

fn simulate(graph: &Graph, loose_pumps: Vec<u64>, mut paths: HashMap<u64, Vec<(u64, Path)>>) {



    for (_from_h, p) in paths.iter_mut() {
        p.sort_by(|a, b|
            (a.1.pressure - a.1.len).cmp(&(b.1.pressure - b.1.len)));
    }

    let mut time = 0;
    let mut pressure_open = 0;
    let mut pressure_released = 0;

    let start_node_h : u64 = 0;


    let mut q = VecDeque::new();
    q.push_front(start);

    let mut passed_nodes: HashSet<u64> = HashSet::new();

    while !q.is_empty() {

        let current_node_h = q.pop_front().unwrap();
        let current_node: Vertex = graph.vertices[current_node_h];
        passed_nodes.insert(current_node_h);

        if current_node.pressure > 0 {
            time += 1;
            pressure_released += pressure_open;
            pressure_open += current_node.pressure;
        }


        let to_paths: &Vec<(u64, Path)> = paths[current_node_h];
        let children = to_paths.iter().filter(
            |&t| passed_nodes.contains(&t.0) && t.1.len < (30 - time)
        ).collect();


        if children.is_empty {
            println!("found: {}", pressure_released + (30 - time) * pressure_open);
            passed_nodes.remove(current_node_h);

        } else {
            for c in children{
                q.push_front(c);
            }
        }





    }

}


fn main() {

    let mut graph: Graph = Graph {
        vertices: HashMap::new(),
        adjacencies: HashMap::new(),
    };

    let mut start_o: Option<Vertex> = None;

    let reader = BufReader::new(File::open("input.txt").unwrap());

    for line in reader.lines().map(|l| l.unwrap()) {

        let splits: Vec<&str> = line.split_whitespace().collect();

        let v = Vertex{id: splits[1].to_string(), pressure: *&splits[4][5..splits[4].len()-1].parse::<usize>().unwrap() };
        graph.vertices.insert(calculate_hash(&v), v);
    }

    let reader = BufReader::new(File::open("input.txt").unwrap());

    for line in reader.lines().map(|l| l.unwrap()) {

        let splits: Vec<&str> = line.split_whitespace().collect();
        let mut hashes = Vec::new();

        for connection in &splits[9..] {
            hashes.push(calculate_hash(&connection.split(',').next().unwrap().to_string()));
        }

        graph.adjacencies.insert(
            calculate_hash(&splits[1].to_string()),
            hashes);
    }

    let mut loose_pumps: Vec<u64> = Vec::new();

    for (h, v) in graph.vertices.iter() {
        if v.pressure > 0 {
            loose_pumps.push(*h);
        }
    }

    let mut paths: HashMap<u64, Vec<(u64, Path)>> = HashMap::new();

    for from in loose_pumps.iter() {

        let mut to_paths = Vec::new();
        for to in loose_pumps.iter() {
            if from != to {

                let vs = calculate_shortest_path(&graph, *from, *to);
                let p = Path{
                    from:*from, to: *to,
                    len: vs.len(),
                    pressure: graph.vertices[to].pressure,
                    vertices: vs
                };
                to_paths.push((*to, p));
            }
        }

        paths.insert(*from, to_paths);
    }
}