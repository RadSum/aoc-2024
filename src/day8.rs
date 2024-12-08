use std::collections::HashMap;

type Position = (isize, isize);

fn prepare_map(lines: &Vec<Vec<char>>) -> HashMap<char, Vec<Position>> {
    let mut positions: HashMap<char, Vec<Position>> = HashMap::new();
    
    for (y, line) in lines.iter().enumerate() {
       for (x, character) in line.iter().enumerate() {
            if *character == '.' {
                continue;
            } 
            positions.entry(*character).or_insert(Vec::new()).push((x as isize, y as isize));
       } 
    }

    positions
}

fn make_antinodes(lines: &mut Vec<Vec<char>>, curr_freq: &Vec<Position>, task1: bool) -> usize {
    let mut new_antinodes = 0;

    for (i, first) in curr_freq.iter().enumerate() {
        for j in (i + 1)..curr_freq.len() {
            let second = &curr_freq[j];

            let (dx, dy) = (second.0 - first.0, second.1 - first.1);
            new_antinodes += check_antinodes(lines, dx, dy, *first, -1, task1); 
            new_antinodes += check_antinodes(lines, dx, dy, *second, 1, task1); 
        }
    }

    new_antinodes
}

fn check_antinodes(lines: &mut Vec<Vec<char>>, dx: isize, dy: isize, start_pos: Position, minus: isize, task1: bool) -> usize {
    let max_x = lines[0].len() as isize;
    let max_y = lines.len() as isize; 

    let (mut lx, mut ly) = (start_pos.0 + if task1 { dx * minus } else { 0 }, start_pos.1 + if task1 { dy * minus } else { 0 });
    let mut res = 0;
    while lx >= 0 && ly >= 0 && lx < max_x && ly < max_y {
        if lines[ly as usize][lx as usize] != '#' {
            res += 1;
            lines[ly as usize][lx as usize] = '#'; 
        } 
        lx += dx * minus;
        ly += dy * minus;
        if task1 {
            break;
        }
    }

    res
}

fn task(task1: bool) -> usize {
    let mut lines = crate::utils::read_file_to_char_vec("day8.in").unwrap();
    let positions = prepare_map(&lines);

    let mut res = 0;
    for (_, freq) in positions.iter() {
        res += make_antinodes(&mut lines, freq, task1);
    }

    res
}

pub fn task1() -> usize {
    task(true) 
}

pub fn task2() -> usize {
    task(false)
}
