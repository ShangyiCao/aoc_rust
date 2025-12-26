use std::env;
use std::fs;

struct Point {
    i: i32,
    j: i32,
}

fn is_inside(i: i32, j: i32, nx: i32, ny: i32) -> bool {
    return 0 <= i && i < nx && 0 <= j && j < ny;
}
fn main() {
    let filename = env::args().nth(1).expect("invalid index");
    let file = fs::read_to_string(filename).expect("fail to read file");
    let lines: Vec<&str> = file.lines().filter(|l| !l.is_empty()).collect();

    let nx = lines[0].len() as i32;
    let ny = lines.len() as i32;

    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut global_count = 0;
    let mut it = 0;
    loop {
        let mut removed_indexes: Vec<Point> = Vec::new();

        for j in 0..ny {
            for i in 0..nx {
                if grid[j as usize][i as usize] == '@' {
                    let mut count = 0;
                    for ii in i - 1..=i + 1 {
                        for jj in j - 1..=j + 1 {
                            if is_inside(ii, jj, nx, ny) {
                                if grid[jj as usize][ii as usize] == '@' {
                                    count += 1;
                                }
                            }
                        }
                    }
                    if count <= 4 {
                        removed_indexes.push(Point { i: i, j: j });
                        global_count += 1;
                    }
                }
            }
        }
        if it == 0 {
            println!("{global_count}");
        }
        if removed_indexes.is_empty() {
            break;
        }
        for Point { i, j } in removed_indexes {
            grid[j as usize][i as usize] = '.';
        }
        it += 1;
    }
    println!("{global_count}");
}
