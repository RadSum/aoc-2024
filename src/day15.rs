use std::fs;

type Direction = (isize, isize);

fn char_to_direction(c: char) -> Direction {
    match c {
        '^' => (0, -1),
        '>' => (1, 0),
        '<' => (-1, 0),
        'v' => (0, 1),
        _   => unreachable!(),
    }
}

fn do_move(map: &mut Vec<Vec<char>>, curr_pos: &mut (isize, isize), dir: Direction) -> () {
    let (next_x, next_y) = ((curr_pos.0 + dir.0) as usize, (curr_pos.1 + dir.1) as usize);
    let next_char = map[next_y][next_x];

    match next_char {
        '#' => return,
        '.' => { 
            map[curr_pos.1 as usize][curr_pos.0 as usize] = '.';
            *curr_pos = (next_x as isize, next_y as isize);
            map[next_y][next_x] = '@';
        }
        'O' => {
            let (mut nx, mut ny) = (next_x as isize, next_y as isize);
            while map[ny as usize][nx as usize] == 'O' {
                nx += dir.0;
                ny += dir.1;
            }
            if map[ny as usize][nx as usize] == '#' {
                return;
            }

            map[ny as usize][nx as usize] = 'O';
            map[next_y as usize][next_x as usize] = '@';
            map[curr_pos.1 as usize][curr_pos.0 as usize] = '.';
            *curr_pos = (next_x as isize, next_y as isize);
        }
        _ => unreachable!(),
    }
}

pub fn task1() -> usize { 
    let filestr = fs::read_to_string("day15.in").unwrap();
    let (map, instructions) = filestr
        .split_once("\n\n")
        .unwrap();
    let mut map_lines = map.split("\n").map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut start_pos: (isize, isize) = (0, 0);
    for (y, line) in map_lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '@' {
                start_pos = (x as isize, y as isize);
            }
        }
    }

    for c in instructions.chars() {
        if c == '\n' {
            continue;
        }
        
        do_move(&mut map_lines, &mut start_pos, char_to_direction(c));
    }
    let mut result = 0;
    for (y, line) in map_lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'O' {
                result += 100 * y + x;
            }
        }
    }

    result
}
