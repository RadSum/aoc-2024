use std::collections::HashMap;

type Position = (isize, isize);

fn make_antinodes(lines: &mut Vec<Vec<char>>, curr_freq: &Vec<Position>) -> usize {
    let mut new_antinodes = 0;
    let max_x = lines[0].len() as isize;
    let max_y = lines.len() as isize; 

    for (i, first) in curr_freq.iter().enumerate() {
        for j in (i + 1)..curr_freq.len() {
            let second = &curr_freq[j];

            let (dx, dy) = (second.0 - first.0, second.1 - first.1);

            let (lx, ly) = (second.0 + dx, second.1 + dy);
            let (sx, sy) = (first.0 - dx, first.1 - dy);

            if lx >= 0 && ly >= 0 && lx < max_x && ly < max_y && lines[ly as usize][lx as usize] != '#' {
                new_antinodes += 1;
                lines[ly as usize][lx as usize] = '#'; 
            }
            if sx >= 0 && sy >= 0 && sx < max_x && sy < max_y && lines[sy as usize][sx as usize] != '#' {
                new_antinodes += 1;
                lines[sy as usize][sx as usize] = '#'; 
            }
        }
    }

    new_antinodes
}


pub fn task1() -> usize {
    let mut lines = crate::utils::read_file_to_lines("day8.in")
        .unwrap()
        .into_iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut positions: HashMap<char, Vec<Position>> = HashMap::new();
    
    for (y, line) in lines.iter().enumerate() {
       for (x, character) in line.iter().enumerate() {
            if *character == '.' {
                continue;
            } 
            positions.entry(*character).or_insert(Vec::new()).push((x as isize, y as isize));
       } 
    }

    let mut res = 0;
    for (_, freq) in positions.iter() {
        res += make_antinodes(&mut lines, freq);
    }

    res
}
