use std::collections::{HashMap};
use std::collections::hash_map::DefaultHasher;
use std::collections::vec_deque::VecDeque;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, BufRead};

struct Graph {
    vertices: HashMap<u64, Vertex>,
    adjacencies: HashMap<u64, Vec<u64>>,
}

#[derive(Clone, Debug, Hash)]
struct Vertex {
    x: i32,
    y: i32,
    height: char
}

impl Vertex {

    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

fn heuristic_score(v: &Vertex, goal: &Vertex) -> i32 {

    (v.x - goal.x).abs() + (v.y - goal.y).abs()
}

fn a_star(graph: &mut Graph, start: &Vertex, target: &Vertex) {

    let mut Q: VecDeque<Vertex> = VecDeque::new();
    let mut g_score: HashMap<u64, i32> = HashMap::new();
    let mut f_score: HashMap<u64, i32> = HashMap::new();
    let mut previous: HashMap<u64, Option<u64>> = HashMap::new();

    for (h, _) in graph.vertices.iter() {
        g_score.insert(*h, std::i32::MAX);
        f_score.insert(*h, std::i32::MAX);
        previous.insert(*h, Option::None);
    }

    g_score.insert(start.calculate_hash(), 0);
    f_score.insert(start.calculate_hash(), heuristic_score(start, target));
    Q.push_front(start.clone());

    while !Q.is_empty() {

        let min_v = Q.iter().min_by(|v1, v2| f_score[&v1.calculate_hash()].cmp(&f_score[&v2.calculate_hash()])).unwrap().clone();

        println!("min_v {:?} hash {}, target {:?} {}", min_v, min_v.calculate_hash(), target, target.calculate_hash());
        println!("neighbours {}, Q len {}", graph.adjacencies.get(&min_v.calculate_hash()).unwrap().len(), Q.iter().len());

        if min_v.calculate_hash() == target.calculate_hash() {

            println!("found path");
            let mut path: Vec<Vertex> = vec![];

            path.push(target.clone());
            let mut prev = &graph.vertices[&previous[&target.calculate_hash()].unwrap()];

            while prev.calculate_hash() != start.calculate_hash() {
                path.push(graph.vertices[&prev.calculate_hash()].clone());
                prev = &graph.vertices[&previous[&prev.calculate_hash()].unwrap()];
            }

            path.push(start.clone());

            for v in path.iter().rev() {
                println!("vertex {:?}", v);
            }

            println!("path length {}", path.len());
        }

        Q.retain(|v| v.calculate_hash() != min_v.calculate_hash());

        for neighbour in graph.adjacencies[&min_v.calculate_hash()].iter().map(|hash| &graph.vertices[hash]) {

            let tentative = g_score[&min_v.calculate_hash()] + 1;

            if tentative < g_score[&neighbour.calculate_hash()] {
                previous.insert(neighbour.calculate_hash(), Option::Some(min_v.calculate_hash()));
                g_score.insert(neighbour.calculate_hash(), tentative);
                f_score.insert(neighbour.calculate_hash(), tentative + heuristic_score(&neighbour, target));

                if !Q.iter().any(|v| v.calculate_hash() == neighbour.calculate_hash()) {
                    Q.push_back(neighbour.clone());
                }
            }
        }
    }
}

fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut graph: Graph = Graph {
        vertices: HashMap::new(),
        adjacencies: HashMap::new(),
    };

    let max_x = reader.lines().nth(0).unwrap().unwrap().len() as i32;
    let mut max_y = 0;

    let mut start_o: Option<Vertex> = None;
    let mut goal_o: Option<Vertex> = None;

    let reader = BufReader::new(File::open("input.txt").unwrap());

    for (y, line) in reader.lines().map(|l| l.unwrap()).enumerate() {

    max_y += 1;

        for (x, c) in line.chars().enumerate() {

            let mut height = c;

            if height == 'S' {
                height = 'a';
                let v = Vertex { x:x as i32, y: y as i32, height: height};
                start_o = Option::Some(v.clone());
                graph.vertices.insert(v.calculate_hash(), v);
            } else if height == 'E' {
                height = 'z';
                let v = Vertex { x:x as i32, y: y as i32, height: height};
                goal_o = Option::Some(v.clone());
                graph.vertices.insert(v.calculate_hash(), v);
            } else{
                let v = Vertex { x:x as i32, y: y as i32, height: height};
                graph.vertices.insert(v.calculate_hash(), v);
            }
        }
    }

    println!("max_x {}, max_y {}", max_x, max_y);

    for (hash, v) in graph.vertices.iter() {

        println!("looking for neighbours on {:?}", v);
        graph.adjacencies.insert(*hash, Vec::new());

        // Right
        if v.x > 0 {
            if let Some((neighbour_hash, neighbour)) = graph.vertices.iter().find(
                |(_h, other_v)| other_v.x == v.x - 1 && other_v.y == v.y && *_h != hash
                    && ((other_v.height as u32) as i32 - (v.height as u32) as i32) < 2){
                println!("found neighbour for {:?}, {:?}", v, neighbour);
                graph.adjacencies.get_mut(&hash).unwrap().push(*neighbour_hash);
            }
        }
        // Left
        if v.x < max_x - 1 {
            if let Some((neighbour_hash, neighbour)) = graph.vertices.iter().find(
                |(_h, other_v)| other_v.x == v.x + 1 && other_v.y == v.y && *_h != hash
                    && ((other_v.height as u32) as i32 - (v.height as u32) as i32) < 2){
                println!("found neighbour for {:?}, {:?}", v, neighbour);
                graph.adjacencies.get_mut(&hash).unwrap().push(*neighbour_hash);
            }
        }
        // Up
        if v.y > 0 {
            if let Some((neighbour_hash, neighbour)) = graph.vertices.iter().find(
                |(_h, other_v)| other_v.y == v.y - 1 && other_v.x == v.x && *_h != hash
                    && ((other_v.height as u32) as i32 - (v.height as u32) as i32) < 2){
                println!("found neighbour for {:?}, {:?}", v, neighbour);
                graph.adjacencies.get_mut(&hash).unwrap().push(*neighbour_hash);
            }
        }
        // Down
        if v.y < max_y - 1 {
            if let Some((neighbour_hash, neighbour)) = graph.vertices.iter().find(
                |(_h, other_v)| other_v.y == v.y + 1 && other_v.x == v.x && *_h != hash
                    && ((other_v.height as u32) as i32 - (v.height as u32) as i32) < 2){
                println!("found neighbour for {:?}, {:?}", v, neighbour);
                graph.adjacencies.get_mut(&hash).unwrap().push(*neighbour_hash);
            }
        }
    }

    a_star(&mut graph, &start_o.unwrap(), &goal_o.unwrap());
}