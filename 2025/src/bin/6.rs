use std::{env, fs, vec};

fn main() {
    let filename = env::args().nth(1).expect("invalid file path");
    let file = fs::read_to_string(filename).expect("unable to read file");
    let lines: Vec<&str> = file.lines().filter(|line| !line.is_empty()).collect();

    let operators: Vec<char> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .collect();

    let mut rows: Vec<Vec<i64>> = Vec::new();
    for line in &lines[..lines.len() - 1] {
        let numbers = line
            .split_whitespace()
            .map(|number| number.parse::<i64>().expect("invalid number"))
            .collect();
        rows.push(numbers);
    }

    let mut cols_str: Vec<String> = vec![String::new(); lines[0].len()];
    for line in &lines[..lines.len() - 1] {
        let chs: Vec<char> = line.chars().collect();
        for (j, ch) in chs.iter().enumerate() {
            cols_str[j].push(*ch);
        }
    }

    let mut cols: Vec<Vec<i64>> = vec![Vec::new(); operators.len()];
    let mut i = 0;
    for str in cols_str {
        if str.trim().is_empty() {
            i += 1;
            continue;
        }
        cols[i].push(str.trim().parse::<i64>().expect("invalid number"));
    }

    let mut results1: Vec<_> = operators
        .iter()
        .map(|operator| if *operator == '+' { 0 } else { 1 })
        .collect();
    let mut results2 = results1.clone();
    for row in rows {
        for (j, num) in row.iter().enumerate() {
            match operators[j] {
                '+' => results1[j] += num,
                '*' => results1[j] *= num,
                _ => panic!("invalid operator"),
            }
        }
    }

    for (i, col) in cols.iter().enumerate() {
        for num in col {
            match operators[i] {
                '+' => results2[i] += num,
                '*' => results2[i] *= num,
                _ => panic!("invalid operator"),
            }
        }
    }

    println!("{}", results1.iter().sum::<i64>());
    println!("{}", results2.iter().sum::<i64>());
}
