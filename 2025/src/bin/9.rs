use std::{env, fs};

#[derive(Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

struct Pair {
    a: Coordinate,
    b: Coordinate,
}

#[derive(PartialEq)]
enum ORIENTATION {
    HORIZONTAL,
    VERTICAL,
}

fn distance(pair: &Pair) -> i64 {
    ((pair.a.x - pair.b.x).abs() + 1) * ((pair.a.y - pair.b.y).abs() + 1)
}

fn orientation(edge: &Pair) -> ORIENTATION {
    if edge.a.y == edge.b.y {
        ORIENTATION::HORIZONTAL
    } else {
        ORIENTATION::VERTICAL
    }
}

fn intersects(edge1: &Pair, edge2: &Pair) -> bool {
    if orientation(edge1) == orientation(edge2) {
        false
    } else {
        if orientation(edge1) == ORIENTATION::HORIZONTAL {
            edge1.a.y > edge2.a.y
                && edge1.a.y < edge2.b.y
                && edge2.a.x > edge1.a.x
                && edge2.a.x < edge1.b.x
        } else {
            intersects(edge2, edge1)
        }
    }
}

fn on_edge(point: &Coordinate, edge: &Pair) -> bool {
    if orientation(edge) == ORIENTATION::HORIZONTAL {
        point.y == edge.a.y && point.x >= edge.a.x && point.x <= edge.b.x
    } else {
        point.x == edge.a.x && point.y >= edge.a.y && point.y <= edge.b.y
    }
}

fn ray_casting(point: &Coordinate, domain_edges: &Vec<Pair>) -> bool {
    if domain_edges.iter().any(|edge| on_edge(point, edge)) {
        return true;
    }
    let mut count = 0;
    for edge in domain_edges
        .iter()
        .filter(|edge| orientation(edge) == ORIENTATION::VERTICAL)
    {
        if point.x < edge.a.x && point.y > edge.a.y && point.y <= edge.b.y {
            count += 1;
        }
    }
    count % 2 != 0
}

fn make_pair(a: &Coordinate, b: &Coordinate) -> Pair {
    if a.x == b.x {
        if a.y < b.y {
            Pair { a: *a, b: *b }
        } else {
            Pair { a: *b, b: *a }
        }
    } else {
        if a.x < b.x {
            Pair { a: *a, b: *b }
        } else {
            Pair { a: *b, b: *a }
        }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("invalid file path");
    let file = fs::read_to_string(filename).expect("unable to read file");
    let mut vertices: Vec<Coordinate> = Vec::new();
    for line in file.lines().filter(|line| !line.is_empty()) {
        let ab: Vec<i64> = line
            .split(',')
            .map(|str| str.parse::<i64>().expect("invalid number"))
            .collect();
        vertices.push(Coordinate { x: ab[0], y: ab[1] });
    }
    let mut pairs: Vec<Pair> = Vec::new();
    for i in 0..vertices.len() {
        for j in i + 1..vertices.len() {
            pairs.push(make_pair(&vertices[i], &vertices[j]));
        }
    }
    pairs.sort_by_key(|pair| distance(pair));
    pairs.reverse();
    println!("{}", distance(&pairs[0]));

    let mut domain_edges: Vec<Pair> = Vec::new();
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        domain_edges.push(make_pair(&vertices[i], &vertices[j]));
    }

    for pair in pairs {
        let a = pair.a;
        let b = pair.b;
        let c = Coordinate { x: a.x, y: b.y };
        let d = Coordinate { x: b.x, y: a.y };
        let rectangle_vertices = [a, b, c, d];
        let rectangle_edges = [
            make_pair(&a, &c),
            make_pair(&a, &d),
            make_pair(&b, &c),
            make_pair(&b, &d),
        ];
        if rectangle_edges.iter().any(|rectangle_edge| {
            domain_edges
                .iter()
                .any(|domain_edge| intersects(domain_edge, rectangle_edge))
        }) {
            continue;
        }
        if rectangle_vertices
            .iter()
            .all(|vertice| ray_casting(&vertice, &domain_edges))
        {
            println!("{}", distance(&pair));
            break;
        }
    }
}
