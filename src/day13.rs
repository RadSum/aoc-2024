use std::fs;
use num_rational::BigRational;
use num_bigint::BigInt;


const TASK2_OFFSET: usize = 10_000_000_000_000;

fn parse_values(line: &str, res_delim: &str) -> (BigInt, BigInt) {
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

    let mut result: BigInt = 0.into();
    for pa in &parsed {
        let lines = pa.split("\n").collect::<Vec<_>>();
        let (x1, y1) = parse_values(lines[0], "+");
        let (x2, y2) = parse_values(lines[1], "+");
        let (mut rx, mut ry) = parse_values(lines[2], "=");

        if task2 {
            rx += TASK2_OFFSET;
            ry += TASK2_OFFSET;
        }

        let equality_ratio = BigRational::new(-y2, x2.clone());
        let new_x: BigRational = <BigInt as Into<BigRational>>::into(x1.clone()) * &equality_ratio + y1; 
        let new_res: BigRational = <BigInt as Into<BigRational>>::into(rx.clone()) * equality_ratio + ry;

        let res_x = new_res / new_x;
        if !res_x.is_integer() {
            continue;
        }

        let res_y: BigRational = (<BigInt as Into<BigRational>>::into(rx) - <BigInt as Into<BigRational>>::into(x1) * &res_x) / x2;
        if !res_y.is_integer() {
            continue;
        }
        result += res_x.numer() * 3 + res_y.numer();
    }

    result.try_into().unwrap()
}


pub fn task1() -> usize {
    task(false)
}

pub fn task2() -> usize {
    task(true)
}
