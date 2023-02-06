use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::collections::vec_deque::VecDeque;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, BufRead};

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
    dist: i32,
    pressure: i32,
    vertices :Vec<u64>
}

#[derive(Clone, Debug)]
struct WalkState {
    path: Vec<u64>,
    time: i32,
    pressure_released: i32,
    pressure_open: i32
}

#[derive(Clone, Debug)]
struct WalkTransition {
    new_node: u64,
    dist: i32
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

const MAX_TIME: i32 = 30;

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

fn simulate(graph: &Graph, paths: &mut HashMap<u64, Vec<(u64, Path)>>, start_node_h: u64) {

    let origin_state = WalkState{ 
        path: vec![], time: 0, pressure_released: 0, pressure_open: 0 };

    let mut q: VecDeque<(WalkState, WalkTransition)> = VecDeque::new();
    q.push_front((origin_state, WalkTransition{new_node: start_node_h, dist: 0}));

    //let mut results: Vec<(i32, Vec<u64>)> = Vec::new();
    let mut best_result: (i32, Vec<u64>) = (0, vec![]); 


    while !q.is_empty() {

        let (old_state, transition) = q.pop_front().unwrap();
        let current_node_h = transition.new_node;
        let current_node: &Vertex = &graph.vertices[&current_node_h];
        let current_state = tick(&old_state, &transition, &current_node);

        paths.get_mut(&transition.new_node).unwrap().sort_by(|a, b|
            compare_paths(&a.1, &b.1, &current_state.time));

        let children: Vec<(u64, &Path)> = (&paths[&current_node_h]).iter().filter_map(
            |a| if current_state.path.contains(&a.0) || a.1.dist > MAX_TIME - current_state.time {None} else {Some((a.0, &a.1))}).collect();

        if children.is_empty() {
            let end_result = current_state.pressure_released + (MAX_TIME - current_state.time) * current_state.pressure_open;

            if end_result > best_result.0 {
                best_result = (end_result, current_state.path.clone());
                println!("Better result found {} Path: {:?}", end_result, current_state.path);
            }
        } else {
            for c in children.iter() {
                q.push_front((current_state.clone(), WalkTransition{ new_node: c.0, dist: c.1.dist }));
            }
        }
    }

    println!("best_result: {} path: {:?}", best_result.0, best_result.1);

}


fn tick(state: &WalkState, transition: &WalkTransition, new_node: &Vertex) -> WalkState {
    let mut w = state.clone();
    w.path.push(transition.new_node);
    w.time += transition.dist;
    w.pressure_released += w.pressure_open * transition.dist;

    if new_node.pressure > 0 {
        w.time += 1;
        w.pressure_released += w.pressure_open;
        w.pressure_open += new_node.pressure;
    }

    w
}


fn compare_paths(a: &Path, b: &Path, time: &i32) -> Ordering {

    let a_res = (MAX_TIME - time - 1 - a.dist) * a.pressure;
    let b_res = (MAX_TIME - time - 1 - b.dist) * b.pressure;

    let res = a_res.cmp(&b_res);
    return res;
}

fn main() {

    let mut graph: Graph = Graph {
        vertices: HashMap::new(),
        adjacencies: HashMap::new(),
    };

    let mut start_o: Option<u64> = None;

    let reader = BufReader::new(File::open("input.txt").unwrap());

    for line in reader.lines().map(|l| l.unwrap()) {

        let splits: Vec<&str> = line.split_whitespace().collect();

        let v = Vertex{id: splits[1].to_string(), pressure: *&splits[4][5..splits[4].len()-1].parse::<i32>().unwrap() };
        if v.id == "AA" {
            start_o = Some(calculate_hash(&v));
        }

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
    loose_pumps.push(start_o.unwrap());

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
                    dist: vs.len() as i32,
                    pressure: graph.vertices[to].pressure,
                    vertices: vs
                };
                to_paths.push((*to, p));
            }
        }

        paths.insert(*from, to_paths);
    }

    simulate(&graph, &mut paths, start_o.unwrap());
}