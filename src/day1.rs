use crate::utils;
use std::collections::HashMap;

fn parse_lines() -> (Vec<usize>, Vec<usize>) {
    let parsed_lines = utils::read_file_to_lines("day1.in").unwrap()
        .into_iter()
        .flat_map(|line| line.split_once(' ').map(|(x, y)| (x.to_owned(), y.to_owned())))
        .map(|(x, y)| { ( x.trim().parse::<usize>().unwrap(), y.trim().parse::<usize>().unwrap() ) });

    let firsts = parsed_lines.clone().map(|(x, _)| x).collect::<Vec<usize>>();
    let seconds = parsed_lines.map(|(_, y)| y).collect::<Vec<usize>>();

    (firsts, seconds)
}

pub fn task1() -> usize {
    let (mut firsts, mut seconds) = parse_lines();
    firsts.sort();
    seconds.sort();

    firsts.into_iter().zip(seconds).map(|(x, y)| x.abs_diff(y)).sum()
}

pub fn task2() -> usize {
    let (firsts, seconds) = parse_lines();
    
    let second_freqs = seconds
        .iter()
        .fold(HashMap::new(), |mut map, val| {
           map.entry(val)
              .and_modify(|frq|*frq += 1)
              .or_insert(1);
           map
        });

    firsts.iter().map(|x| *&second_freqs.get(x).unwrap_or(&0) * x).sum()
}
