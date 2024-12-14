use std::fs;

const TASK2_OFFSET: isize = 10_000_000_000_000;

fn parse_values(line: &str, res_delim: &str) -> (isize, isize) {
    let (x1, y1) = &line.split_once(": ").unwrap().1.split_once(", ").unwrap();
    let x1 = x1.split_once(res_delim).unwrap().1.parse().unwrap();
    let y1 = y1.split_once(res_delim).unwrap().1.parse().unwrap();

    (x1, y1)
}

fn task(task2: bool) -> usize {
    let binding = fs::read_to_string("day13.in")
        .unwrap();
    let parsed: Vec<&str> = binding
        .trim()
        .split("\n\n")
        .collect::<Vec<_>>();

    let mut result = 0;
    for pa in &parsed {
        let lines = pa.split("\n").collect::<Vec<_>>();
        let (x1, y1) = parse_values(lines[0], "+");
        let (x2, y2) = parse_values(lines[1], "+");
        let (mut rx, mut ry) = parse_values(lines[2], "=");

        if task2 {
            rx += TASK2_OFFSET;
            ry += TASK2_OFFSET;
        }

        // using cramer's rule to solve https://en.wikipedia.org/wiki/Cramer's_rule
        if (y2 * rx - x2 * ry) % (y2 * x1 - x2 * y1) == 0 && (x1 * ry - y1 * rx) % (y2 * x1 - x2 * y1) == 0 {
            result += 3 * ((y2 * rx - x2 * ry) / (y2 * x1 - x2 * y1)) + (x1 * ry - y1 * rx) / (y2 * x1 - x2 * y1);
        }
    }

    result as usize
}


pub fn task1() -> usize {
    task(false)
}

pub fn task2() -> usize {
    task(true)
}
