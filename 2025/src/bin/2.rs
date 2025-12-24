use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("provide input filename");
    let input = fs::read_to_string(filename).expect("failed to read input file");

    let mut counter1: i64 = 0;
    let mut counter2: i64 = 0;
    for range in input.trim().split(',') {
        let (a, b) = range.split_once('-').expect("invalid range");
        let a: i64 = a.parse().expect("invalid number");
        let b: i64 = b.parse().expect("invalid number");
        for x in a..=b {
            let x_str = x.to_string();
            let len = x_str.len();
            let (first, second) = x_str.split_at(len / 2);
            if len % 2 == 0 && first == second {
                counter1 += x;
            }

            for period in 1..=len / 2 {
                if len % period != 0 {
                    continue;
                }
                let chunks: Vec<&[u8]> = x_str.as_bytes().chunks(period).collect();
                let first_chunk = chunks[0];
                if chunks.iter().all(|chunk| *chunk == first_chunk) {
                    counter2 += x;
                    break;
                }
            }
        }
    }
    println!("{counter1}");
    println!("{counter2}");
}
