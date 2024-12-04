type Position = (isize, isize);
type Direction = (isize, isize);

fn check_direction(lines: &Vec<Vec<char>>, curr_pos: Position, direction: Direction, max_x: isize, max_y: isize) -> bool {
    let check_arr: [char; 3] = ['M', 'A', 'S'];
    for i in 1..4 {
        let next_y = curr_pos.1 + i * direction.1;
        let next_x = curr_pos.0 + i * direction.0;
        if next_x < 0 || next_y < 0 || next_x >= max_x || next_y >= max_y {
            return false;
        }
        if lines[next_y as usize][next_x as usize] != check_arr[(i as usize) - 1] {
            return false;
        }
    }
    true
}

pub fn task1() -> usize {
    let lines = crate::utils::read_file_to_lines("day4.in").unwrap();
    let lines = lines.iter().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut xmas_count = 0;

    let mut curr_y = 0;
    for line in &lines {
        for (curr_x, char) in line.iter().enumerate() {
            if *char != 'X' {
                continue;    
            }
            for dx in -1..2 {
                for dy in -1..2 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    if check_direction(&lines, (curr_x as isize, curr_y), (dx, dy), line.len() as isize, lines.len() as isize) {
                        xmas_count += 1;
                    }
               }
            }
        }
        curr_y += 1;
    }

    xmas_count
}

fn check_xmas(lines: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let lt = lines[y + 1][x - 1];
    let ld = lines[y - 1][x - 1];
    let rt = lines[y + 1][x + 1];
    let rd = lines[y - 1][x + 1];

    let diag_one = rt == 'M' && ld == 'S' || rt == 'S' && ld == 'M';
    let diag_two = lt == 'M' && rd == 'S' || lt == 'S' && rd == 'M';

    diag_one && diag_two
}

pub fn task2() -> usize {
    let lines = crate::utils::read_file_to_lines("day4.in").unwrap();
    let lines = lines.iter().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut xmas_count = 0;

    for (curr_y, line) in lines.iter().enumerate().skip(1).take(&lines.len() - 2) {
        for (curr_x, char) in line.iter().enumerate().skip(1).take(&line.len() - 2) {
            if *char == 'A' && check_xmas(&lines, curr_x, curr_y) {
                xmas_count += 1;
            } 
        }
    }

    xmas_count
}
