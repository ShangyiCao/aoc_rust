use std::env;
use std::fs;

fn find_combo(line: &str, nx: i32, digits: i32) -> i64 {
    let mut maxes: Vec<i32> = vec![-1; digits as usize];
    let mut indexes: Vec<i32> = vec![-1; digits as usize];
    for k in 0..digits {
        let mut i = 0;
        if k > 0 {
            i = indexes[(k - 1) as usize] + 1;
        }
        while i < nx - (digits - k) + 1 {
            let x: i32 = line
                .chars()
                .nth(i as usize)
                .expect("invalid index")
                .to_digit(10)
                .expect("not a digit") as i32;
            if x > maxes[k as usize] {
                maxes[k as usize] = x;
                indexes[k as usize] = i;
            }
            i += 1;
        }
    }
    let mut result: i64 = 0;
    for i in 0..digits {
        result = result * 10 + maxes[i as usize] as i64;
    }
    return result;
}

fn main() {
    let filename = env::args().nth(1).expect("file name not provided");
    let input = fs::read_to_string(filename).expect("failed to read input file");

    let mut counter1: i64 = 0;
    let mut counter2: i64 = 0;
    for line in input.lines() {
        counter1 += find_combo(line, line.len() as i32, 2);
        counter2 += find_combo(line, line.len() as i32, 12);
    }
    println!("{counter1}");
    println!("{counter2}");
}
