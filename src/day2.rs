use crate::utils;

fn parse_line(line: &str) -> Vec<usize> {
    line.split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn check_line(parsed_line: &[usize], relaxed: bool) -> bool {
    let mut all_ascending: bool = true;
    let mut all_descending: bool = true;

    let mut last: Option<usize> = None;
    for elem in parsed_line.iter() {
        if relaxed && *elem == usize::MAX {
            continue;
        }
        if last.is_none() {
            last = Some(*elem);
            continue;
        }

        let last_un = last.unwrap();

        let diff = last_un.abs_diff(*elem);
        if diff < 1 || diff > 3 {
            return false;
        }

        if last_un < *elem {
            all_descending = false;
        } else {
            all_ascending = false;
        }

        last = Some(*elem);
    }
    all_ascending ^ all_descending
}

pub fn task1() -> usize {
    let mut result = 0;
    for line in utils::read_file_to_lines("day2.in").unwrap() {
        let parsed_line = parse_line(&line);
        if check_line(&parsed_line, false) {
            result += 1;
        }
    }

    result
}

pub fn task2() -> usize {
    let mut result = 0;
    for line in utils::read_file_to_lines("day2.in").unwrap() {
        let mut parsed_line = parse_line(&line);
        for i in 0..parsed_line.len() {
            let curr = parsed_line[i];
            parsed_line[i] = usize::MAX;
            if check_line(&parsed_line, true) {
                result += 1;
                break;
            }
            parsed_line[i] = curr;
        }
    }

    result
}
