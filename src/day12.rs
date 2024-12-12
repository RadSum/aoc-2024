#[derive(Debug)]
struct CharPosition {
    c: char,
    visited: bool,
}

impl CharPosition {
    fn new(c: char) -> Self {
        Self { c, visited: false } 
    }
}

fn dfs_char(lines: &mut Vec<Vec<CharPosition>>, curr_pos: (usize, usize), curr_area: &mut usize, curr_perimeter: &mut usize, maxs: (usize, usize)) -> () {
    let (max_x, max_y) = maxs;
    let (curr_x, curr_y) = curr_pos;
    let char_pos: &mut CharPosition = &mut lines[curr_y][curr_x];
    if char_pos.visited == true {
        return;
    }

    *curr_area += 1;
    char_pos.visited = true;

    if curr_x > 0 {
        if *&lines[curr_y][curr_x - 1].c != *&lines[curr_y][curr_x].c {
            *curr_perimeter += 1;
        } else {
            dfs_char(lines, (curr_x - 1, curr_y), curr_area, curr_perimeter, maxs);
        }
    } else {
        *curr_perimeter += 1;
    }

    if curr_y > 0 {
        if *&lines[curr_y - 1][curr_x].c != *&lines[curr_y][curr_x].c {
            *curr_perimeter += 1;
        } else {
            dfs_char(lines, (curr_x, curr_y - 1), curr_area, curr_perimeter, maxs);
        }
    } else {
        *curr_perimeter += 1;
    }

    if curr_x < max_x {
        if *&lines[curr_y][curr_x + 1].c != *&lines[curr_y][curr_x].c {
            *curr_perimeter += 1;
        } else {
            dfs_char(lines, (curr_x + 1, curr_y), curr_area, curr_perimeter, maxs);
        }
    } else {
        *curr_perimeter += 1;
    }

    if curr_y < max_y {
        if *&lines[curr_y + 1][curr_x].c != *&lines[curr_y][curr_x].c {
            *curr_perimeter += 1;
        } else {
            dfs_char(lines, (curr_x, curr_y + 1), curr_area, curr_perimeter, maxs);
        }
    } else {
        *curr_perimeter += 1;
    }

}

pub fn task1() -> usize {
    let mut lines = crate::utils::read_file_to_char_vec("day12.in")
        .unwrap()
        .into_iter()
        .map(|line| line
            .into_iter()
            .map(CharPosition::new)
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_x = *&lines[0].len();
    let max_y = *&lines.len();
    
    let mut result = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if lines[y][x].visited {
                continue;
            }

            let mut curr_perimeter = 0;
            let mut curr_area = 0;
            dfs_char(&mut lines, (x, y), &mut curr_area, &mut curr_perimeter, (max_x - 1, max_y - 1));
            result += curr_perimeter * curr_area;
        }
    }

    result
}
