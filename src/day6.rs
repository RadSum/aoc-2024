type Position = (isize, isize);

enum Direction{
    Up,
    Down,
    Left, 
    Right
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        } 
    }

    fn to_coords(&self) -> Position {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }
}

fn find_start(lines: &Vec<Vec<char>>) -> Position {
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == '^' {
                return (x as isize, y as isize);
            }
        }
    }
    unreachable!()
}

pub fn task1() -> usize {
    let mut lines = crate::utils::read_file_to_lines("day6.in")
        .unwrap()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_x = *&lines[0].len() as isize;
    let max_y = *&lines.len() as isize;
    let mut curr_position = find_start(&lines);
    let mut curr_direction = Direction::Up;
    let mut positions = 1;
    loop {
        let (dx, dy) = curr_direction.to_coords();
        let (next_x, next_y) = (curr_position.0 + dx, curr_position.1 + dy); 
        if next_x < 0 || next_y < 0 || next_x >= max_x || next_y >= max_y {
            break;
        }
        let curr_pos_char = *&lines[next_y as usize][next_x as usize];
        if curr_pos_char == '#' {
            curr_direction = curr_direction.turn_right();
            continue;
        }
        if curr_pos_char == '.' {
            positions += 1;
        }
        lines[next_y as usize][next_x as usize] = 'X';
        curr_position = (next_x, next_y);
    }

    positions
}
