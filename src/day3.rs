use std::fs;
use regex::Regex;

pub fn task1() -> usize {
    let file_str = fs::read_to_string("day3.in").unwrap();
    
    let mul_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut sum = 0;
    for (x, y) in mul_regex
            .captures_iter(&file_str)
            .map(|c| c.extract())
            .map(|(_, [first, second])| (first.parse::<usize>().unwrap(), second.parse::<usize>().unwrap())) {
        sum += x * y;
    } 
    
    sum
}

pub fn task2() -> usize {
    let file_str = fs::read_to_string("day3.in").unwrap();

    let do_regex = Regex::new(r"(?<mul>mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();

    let mut mul_enabled = true;
    let mut sum = 0;
    for caps in do_regex.captures_iter(&file_str) {
        if caps.name("do").is_some() {
            mul_enabled = true;
        } else if caps.name("dont").is_some() {
            mul_enabled = false
        } else {
            if !mul_enabled {
                continue;
            }

            // without error checking, because AoC input will be valid
            let x = caps.name("x").unwrap().as_str().parse::<usize>().unwrap();
            let y = caps.name("y").unwrap().as_str().parse::<usize>().unwrap();
            sum += x * y;
        }
    }

    sum
}
