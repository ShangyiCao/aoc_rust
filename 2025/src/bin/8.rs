use std::arch::naked_asm;
use std::collections::HashSet;
use std::fs::create_dir;
use std::{env, fs};

#[derive(Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

struct Pair {
    a: usize,
    b: usize,
}

fn dfs(i: usize, graph: &Vec<Vec<usize>>, visited: &mut HashSet<usize>, count: &mut usize) {
    visited.insert(i);
    *count += 1;
    for j in &graph[i] {
        if !visited.contains(j) {
            dfs(*j, graph, visited, count);
        }
    }
}

fn find_circuits(
    graph: &Vec<Vec<usize>>,
    coordinates: &Vec<Coordinate>,
    circuit_sizes: &mut Vec<usize>,
) {
    let mut visited: HashSet<usize> = HashSet::new();
    for i in 0..coordinates.len() {
        if !visited.contains(&i) {
            let mut count = 0;
            dfs(i, &graph, &mut visited, &mut count);
            circuit_sizes.push(count);
        }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("invalid file path");
    let file = fs::read_to_string(filename).expect("unable to read file");
    let mut coordinates: Vec<Coordinate> = Vec::new();
    for line in file.lines().filter(|line| !line.is_empty()) {
        let coordinate: Vec<i64> = line
            .split(",")
            .map(|str| str.parse::<i64>().expect("invalid number"))
            .collect();
        coordinates.push(Coordinate {
            x: coordinate[0],
            y: coordinate[1],
            z: coordinate[2],
        });
    }

    let mut pairs: Vec<Pair> = Vec::new();
    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            pairs.push(Pair { a: i, b: j });
        }
    }
    pairs.sort_by_key(|pair| {
        let coord1 = &coordinates[pair.a];
        let coord2 = &coordinates[pair.b];
        let dx = coord1.x - coord2.x;
        let dy = coord1.y - coord2.y;
        let dz = coord1.z - coord2.z;
        dx * dx + dy * dy + dz * dz
    });

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); coordinates.len()];

    for (i, pair) in pairs.iter().enumerate() {
        graph[pair.a].push(pair.b);
        graph[pair.b].push(pair.a);
        let mut circuit_sizes: Vec<usize> = Vec::new();
        find_circuits(&graph, &coordinates, &mut circuit_sizes);
        circuit_sizes.sort();
        circuit_sizes.reverse();
        if i == 999 {
            println!("{}", circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]);
        }
        if circuit_sizes[0] == coordinates.len() {
            println!("{}", coordinates[pair.a].x * coordinates[pair.b].x);
            break;
        }
    }
}
