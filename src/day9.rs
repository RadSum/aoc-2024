use std::fs;

pub fn task1() -> usize {
    let line: Vec<_> = fs::read_to_string("day9.in").unwrap().trim().chars().collect();

    let mut res_index = 0; 
    let mut last_file = *&line.len() - 1;
    if *&line.len() % 2 == 0 {
        last_file -= 1;
    }
    let mut last_file_val = last_file / 2;
    let mut last_remaining = **&line.last().unwrap() as usize - '0' as usize;
    let mut result = 0;
    let upper_count: usize = line.iter().map(|x| *x as usize - '0' as usize).step_by(2).sum();

    for (i, curr_count) in line.iter().enumerate() {
        let curr_count: usize = *curr_count as usize - '0' as usize;

        for _ in 0..curr_count {
            if i % 2 == 0 {
                result += res_index * i / 2;
            } else {
                if last_remaining == 0 {
                    last_file -= 2;
                    last_file_val -= 1;
                    last_remaining = line[last_file] as usize - '0' as usize;
                }  
                result += res_index * last_file_val; 
                last_remaining -= 1;
            }
            res_index += 1;
            if res_index == upper_count {
                return result;
            }
        }
    }

    unreachable!()
}
