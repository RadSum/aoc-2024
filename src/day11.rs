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

fn simulate_rock_rec(number: usize, generation: usize, to_gen: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if generation == to_gen {
        return 1;
    }

    if let Some(&memoed) = memo.get(&(number, generation)) {
        return memoed;
    }

    if number == 0 {
        let curr_res = simulate_rock_rec(1, generation + 1, to_gen, memo);
        memo.insert((number, generation), curr_res);
        return curr_res;
    }

    let digs = count_digits(number);
    if digs % 2 == 0 {
        let (first, second) = split_number(number, digs);
        let first_res = simulate_rock_rec(first, generation + 1, to_gen, memo);
        let second_res = simulate_rock_rec(second, generation + 1, to_gen, memo);
        memo.insert((number, generation), first_res + second_res);
        return first_res + second_res;
    }

    let res = simulate_rock_rec(number * 2024, generation + 1, to_gen, memo);
    memo.insert((number, generation), res);
    return res;
}

fn simulate_rock(number: usize, memo: &mut HashMap<(usize, usize), usize>, generation: usize) -> usize {
    simulate_rock_rec(number, 0, generation, memo)
}

fn task(generations: usize) -> usize {
    let numbers = crate::utils::read_file_to_lines("day11.in")
        .unwrap()[0]
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut memo = HashMap::new();

    let mut result = 0;
    for num in &numbers {
        result += simulate_rock(*num, &mut memo, generations);
    }
    
    result
}

pub fn task1() -> usize {
    task(25)
}
pub fn task2() -> usize {
    task(75)
}
