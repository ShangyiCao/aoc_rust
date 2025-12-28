use std::collections::{HashMap, HashSet};
use std::{env, fs};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coordinate {
    i: i32,
    j: i32,
}

fn count_splits(
    i: i32,
    j: i32,
    nx: i32,
    ny: i32,
    grid: &Vec<Vec<char>>,
    counter: &mut i32,
    visited: &mut HashSet<Coordinate>,
) {
    visited.insert(Coordinate { i: i, j: j });
    if grid[j as usize][i as usize] == '^' {
        *counter += 1;
        if i - 1 >= 0 && !visited.contains(&Coordinate { i: i - 1, j: j }) {
            count_splits(i - 1, j, nx, ny, grid, counter, visited);
        }
        if i + 1 < nx && !visited.contains(&Coordinate { i: i + 1, j: j }) {
            count_splits(i + 1, j, nx, ny, grid, counter, visited);
        }
        return;
    } else {
        if j == ny - 1 {
            return;
        } else {
            if !visited.contains(&Coordinate { i: i, j: j + 1 }) {
                count_splits(i, j + 1, nx, ny, grid, counter, visited);
            }
            return;
        }
    }
}

fn count_timeline(
    i: i32,
    j: i32,
    nx: i32,
    ny: i32,
    grid: &Vec<Vec<char>>,
    record: &mut HashMap<Coordinate, i64>,
) -> i64 {
    if record.contains_key(&Coordinate { i: i, j: j }) {
        return *record.get(&Coordinate { i: i, j: j }).unwrap();
    }

    if grid[j as usize][i as usize] == '^' {
        let mut sum: i64 = 0;
        if i - 1 >= 0 {
            sum += count_timeline(i - 1, j, nx, ny, grid, record);
        }
        if i + 1 < nx {
            sum += count_timeline(i + 1, j, nx, ny, grid, record);
        }
        record.insert(Coordinate { i: i, j: j }, sum);
        sum
    } else {
        if j == ny - 1 {
            return 1;
        }
        let result = count_timeline(i, j + 1, nx, ny, grid, record);
        record.insert(Coordinate { i: i, j: j }, result);
        result
    }
}

fn main() {
    let filename = env::args().nth(1).expect("invalid file path");
    let file = fs::read_to_string(filename).expect("unable to read file");
    let lines: Vec<&str> = file.lines().filter(|line| !line.is_empty()).collect();

    let grid = lines.iter().map(|line| line.chars().collect()).collect();

    let nx = lines[0].len() as i32;
    let ny = lines.len() as i32;

    let start = file.find("S").expect("unable to find S") as i32;
    let is = start % (nx + 1);
    let js = start / (nx + 1);

    let mut counter1 = 0;
    let mut visited: HashSet<Coordinate> = HashSet::with_capacity((nx * ny * 2) as usize);
    count_splits(is, js, nx, ny, &grid, &mut counter1, &mut visited);
    println!("{counter1}");

    let mut record: HashMap<Coordinate, i64> = HashMap::with_capacity((nx * ny * 2) as usize);
    let counter2 = count_timeline(is, js, nx, ny, &grid, &mut record);
    println!("{counter2}");
}
