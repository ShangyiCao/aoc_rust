use std::cmp;
use std::env;
use std::fs;

#[derive(Clone, Copy, Debug)]
struct Range {
    a: i64,
    b: i64,
}

fn disjoint(range1: &Range, range2: &Range) -> bool {
    range1.b < range2.a || range1.a > range2.b
}

fn merge_ranges(range1: &mut Range, range2: Range) {
    range1.a = cmp::min(range1.a, range2.a);
    range1.b = cmp::max(range1.b, range2.b);
}

fn main() {
    let filename = env::args().nth(1).expect("invalid file name");
    let file = fs::read_to_string(filename).expect("unable to read file");

    let (ranges_str, ingredients_str): (&str, &str) = file
        .split_once("\n\n")
        .expect("input must contain a blank line separator");

    let mut ranges: Vec<Range> = Vec::new();
    for line in ranges_str.lines() {
        if line.is_empty() {
            continue;
        }
        let (a, b) = line.split_once("-").expect("invalid range");
        ranges.push(Range {
            a: a.parse::<i64>().expect("not a valid number"),
            b: b.parse::<i64>().expect("not a valid number"),
        })
    }

    let mut ingredients: Vec<i64> = Vec::new();
    for line in ingredients_str.lines() {
        if line.is_empty() {
            continue;
        }
        ingredients.push(line.parse::<i64>().expect("invalid number"));
    }

    let mut counter1 = 0;
    for ingredient in ingredients {
        if ranges
            .iter()
            .any(|range| range.a <= ingredient && range.b >= ingredient)
        {
            counter1 += 1;
        }
    }
    println!("{counter1}");

    loop {
        let mut removed: Vec<bool> = vec![false; ranges.len()];
        let mut merged_ranges: Vec<Range> = Vec::new();
        for i in 0..ranges.len() {
            if removed[i] {
                continue;
            }
            for j in i + 1..ranges.len() {
                if removed[j] {
                    continue;
                }
                if !disjoint(&ranges[i], &ranges[j]) {
                    let range = ranges[j];
                    merge_ranges(&mut ranges[i], range);
                    removed[j] = true;
                }
            }
            merged_ranges.push(Range {
                a: ranges[i].a,
                b: ranges[i].b,
            });
        }
        if ranges.len() == merged_ranges.len() {
            break;
        }
        ranges = merged_ranges;
    }

    let mut counter2: i64 = 0;
    for range in ranges {
        counter2 += range.b - range.a + 1;
    }
    println!("{counter2}");
}
