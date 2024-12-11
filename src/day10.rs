
fn make_trailhead_rec(lines: &Vec<Vec<char>>, curr_pos: (usize, usize), curr_val: usize, maxs: (usize, usize), all_res: &mut usize, non_dup: &mut usize, used: &mut Vec<(usize, usize)>) -> () {
    if curr_val == 9 {
        *all_res += 1;
        if used.contains(&curr_pos) {
            return;
        }
        *non_dup += 1;
        used.push(curr_pos);
        return;
    }

    let (x, y) = curr_pos;
    if y > 0 && (lines[y - 1][x] as usize - '0' as usize) == curr_val + 1 {
        make_trailhead_rec(lines, (x, y - 1), curr_val + 1, maxs, all_res, non_dup, used);
    }
    if x > 0 && (lines[y][x - 1] as usize - '0' as usize) == curr_val + 1 {
        make_trailhead_rec(lines, (x - 1, y), curr_val + 1, maxs, all_res, non_dup, used);
    }
    if y < maxs.1 && (lines[y + 1][x] as usize - '0' as usize) == curr_val + 1 {
        make_trailhead_rec(lines, (x, y + 1), curr_val + 1, maxs, all_res, non_dup, used);
    }
    if x < maxs.0 && (lines[y][x + 1] as usize - '0' as usize) == curr_val + 1 {
        make_trailhead_rec(lines, (x + 1, y), curr_val + 1, maxs, all_res, non_dup, used);
    }
}

fn make_trailhead(lines: &Vec<Vec<char>>, start_pos: (usize, usize)) -> (usize, usize) {
    let max_x = *&lines[0].len();
    let max_y = *&lines.len();
    let mut curr_res = 0;
    let mut non_dup = 0;
    let mut used_vec = Vec::new();

    make_trailhead_rec(lines, start_pos, 0, (max_x - 1, max_y - 1), &mut curr_res, &mut non_dup, &mut used_vec);

    (non_dup, curr_res)
}

pub fn both_task() -> (usize, usize) {
    let lines = crate::utils::read_file_to_char_vec("day10.in").unwrap();

    let mut result_task1 = 0;
    let mut result_task2 = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '0' {
                continue;
            }

            let (task1, task2) = make_trailhead(&lines, (x, y));
            result_task1 += task1;
            result_task2 += task2;
        }
    }

    (result_task1, result_task2)
}

