use std::collections::HashMap;

fn split_number(number: usize, digits: usize) -> (usize, usize) {
    let rem = 10_usize.pow(digits as u32/2);
    let first = number / rem;
    let second = number % rem;

    (first, second)
}

fn count_digits(number: usize) -> usize {
    let mut digs = 0;
    let mut num = number;
    while num > 0 {
        digs += 1;
        num /= 10;
    }
    digs
}

fn simulate_rock_rec(number: usize, generation: usize, to_gen: usize, res: &mut usize, map: &mut HashMap<(usize, usize), usize>) -> () {
    if generation == to_gen {
        *res += 1;
        return;
    }

    if map.contains_key(&(number, generation)) {
        *res += map.get(&(number, generation)).unwrap();
        return;
    }

    if number == 0 {
        let mut curr_res = 0;
        simulate_rock_rec(1, generation + 1, to_gen, &mut curr_res, map);
        *res += curr_res;
        map.insert((number, generation), curr_res);
        return;
    }
    let digs = count_digits(number);
    if digs % 2 == 0 {
        let (first, second) = split_number(number, digs);
        let mut first_res = 0;
        let mut second_res = 0;
        simulate_rock_rec(first, generation + 1, to_gen, &mut first_res, map);
        simulate_rock_rec(second, generation + 1, to_gen, &mut second_res, map);
        map.insert((number, generation), first_res + second_res);
        *res += first_res;
        *res += second_res;
        return;
    }

    let mut _res = 0;
    simulate_rock_rec(number * 2024, generation + 1, to_gen, &mut _res, map);
    map.insert((number, generation), _res);
    *res += _res;
}

fn simulate_rock(number: usize, map: &mut HashMap<(usize, usize), usize>, generation: usize) -> usize {
    let mut result = 0;
    simulate_rock_rec(number, 0, generation, &mut result, map);
    result
}

fn task(generations: usize) -> usize {
    let numbers = crate::utils::read_file_to_lines("day11.in")
        .unwrap()[0]
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut map = HashMap::new();

    let mut result = 0;
    for num in &numbers {
        result += simulate_rock(*num, &mut map, generations);
    }
    
    result
}

pub fn task1() -> usize {
    task(25)
}
pub fn task2() -> usize {
    task(75)
}
