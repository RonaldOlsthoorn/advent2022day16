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
    pressure: i32
}

struct Path {
    from: u64,
    to: u64,
    len: usize,
    pressure: i32,
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
    let mut Q: HashSet<u64> = HashSet::new();

    for (h, v) in graph.vertices.iter(){
        dist.insert(*h, usize::MAX);
        Q.insert(*h);
    }

    dist.insert(start, 0);

    while !Q.is_empty() {

        let u = *Q.iter().min_by(|&h1, &h2| dist[h1].cmp(&dist[h2])).unwrap();

        if u == end {

            let mut path: Vec<u64> = Vec::new();
            let mut current = end;

            while current != start {
                path.push(current);
                current = prev[&current];
            }

            return path;
        }

        Q.remove(&u);

        for neighbour in graph.adjacencies[&u].iter().filter(|h| Q.contains(h)) {
            let alt = dist[&u] + 1;

            if alt < dist[neighbour] {
                dist.insert(*neighbour, alt);
                prev.insert(*neighbour, u);
            }
        }
    }

    return vec![];
}

fn simulate(graph: &Graph, loosePumps: Vec<u64>, mut paths: HashMap<u64, Vec<(u64, Path)>>) {

    let mut time = 0;

    for (from_h, p) in paths.iter_mut() {
        p.sort_by();
    }


}


fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut graph: Graph = Graph {
        vertices: HashMap::new(),
        adjacencies: HashMap::new(),
    };

    let mut start_o: Option<Vertex> = None;

    let reader = BufReader::new(File::open("input.txt").unwrap());

    for line in reader.lines().map(|l| l.unwrap()) {

        let splits: Vec<&str> = line.split_whitespace().collect();

        let v = Vertex{id: splits[1].to_string(), pressure: *&splits[4][5..splits[4].len()-1].parse::<i32>().unwrap() };
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

    let mut loosePumps: Vec<u64> = Vec::new();

    for (h, v) in graph.vertices.iter() {
        if v.pressure > 0 {
            loosePumps.push(*h);
        }
    }

    let mut paths: HashMap<u64, Vec<(u64, Path)>> = HashMap::new();

    for from in loosePumps.iter() {

        let mut toPaths = Vec::new();
        for to in loosePumps.iter() {
            if from != to {
                let p = Path{
                    from:*from, to: *to, pressure: graph.vertices[to].pressure,
                    vertices: calculate_shortest_path(&graph, *from, *to)
                };
                toPaths.push((*to, p));
            }
        }

        paths.insert(*from, toPaths);
    }
}