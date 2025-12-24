use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("provide input filename");
    let input = fs::read_to_string(filename).expect("failed to read input file");
    let mut x0: i32 = 50;
    let mut count1: i32 = 0;
    let mut count2: i32 = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (c, d) = line.split_at(1);
        let d: i32 = d.parse().expect("failed to parse number");
        match c {
            "R" => {
                x0 += d;
                count2 += x0 / 100;
            }
            "L" => {
                if x0 - d <= 0 {
                    if x0 == 0 {
                        count2 += (d - x0) / 100;
                    } else {
                        count2 += (d - x0) / 100 + 1;
                    }
                }
                x0 -= d;
            }
            _ => println!("Unknown operation: {}", c),
        }
        x0 = x0.rem_euclid(100);
        count1 += (x0 == 0) as i32;
    }
    println!("{count1}");
    println!("{count2}");
}
