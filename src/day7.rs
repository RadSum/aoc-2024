fn parse_line(line: &str) -> (usize, Vec<usize>) {
    let (result, rest) = line.split_once(": ").unwrap();
    let rest = rest.trim().split(" ").map(|x| x.parse().unwrap());

    (result.parse().unwrap(), rest.collect())
}

fn is_valid(result: usize, problem: &Vec<usize>, index: usize, curr_result: usize, task2: bool) -> bool {
    if curr_result > result {
        return false;
    }

    if index == problem.len() {
        return result == curr_result;
    }

    if is_valid(result, problem, index + 1, curr_result * problem[index], task2) {
        return true;
    }
    if is_valid(result, problem, index + 1, curr_result + problem[index], task2) {
        return true;
    }

    if !task2 {
        return false;
    }

    let num_digits = num_of_digits(problem[index]);
    let mut new = curr_result;
    new *= 10_usize.pow(num_digits as u32);
    new += problem[index];
    if is_valid(result, problem, index + 1, new, task2) {
        return true;
    }

    false
}

fn num_of_digits(mut num: usize) -> usize {
    let mut num_of_digs = 0;

    while num > 0 {
        num_of_digs += 1;
        num /= 10;
    }

    num_of_digs
}

fn task(task2: bool) -> usize {
    let lines = crate::utils::read_file_to_lines("day7.in").unwrap(); 

    let mut res = 0;
    for line in lines {
        let (result, problem) = parse_line(&line);
        if is_valid(result, &problem, 1, *&problem[0], task2) {
            res += result;
        }
    }

    res
}

pub fn task1() -> usize {
    task(false) 
}

pub fn task2() -> usize {
    task(true) 
}
