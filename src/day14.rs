#[derive(Debug)]
struct RobotInfo {
    start: (isize, isize),
    velocity: (isize, isize),
}

const TILE_WIDTH: usize = 101;
const TILE_HEIGHT: usize = 103;
const MIDDLE_INDEX_WIDTH: usize = TILE_WIDTH / 2;
const MIDDLE_INDEX_HEIGHT: usize = TILE_HEIGHT / 2;
const SECONDS: usize = 100;

fn parse_line(line: &str) -> RobotInfo {
    let (start, vel) = line.split_once(" ").unwrap();
    let (sx, sy) = start.split_once("=").unwrap().1.split_once(",").unwrap();
    let (vx, vy) = vel.split_once("=").unwrap().1.split_once(",").unwrap();

    RobotInfo {
        start: (sx.parse().unwrap(), sy.parse().unwrap()),
        velocity: (vx.parse().unwrap(), vy.parse().unwrap())
    }
}

fn simulate_robot(info: RobotInfo, quadrants: &mut (usize, usize, usize, usize)) -> () {
    let final_x = (info.start.0 + SECONDS as isize * info.velocity.0).rem_euclid(TILE_WIDTH as isize) as usize;
    let final_y = (info.start.1 + SECONDS as isize * info.velocity.1).rem_euclid(TILE_HEIGHT as isize) as usize;

    if final_x == MIDDLE_INDEX_WIDTH || final_y == MIDDLE_INDEX_HEIGHT {
        return;
    }

    if final_x < MIDDLE_INDEX_WIDTH && final_y < MIDDLE_INDEX_HEIGHT {
        quadrants.0 += 1;
    }
    if final_x < MIDDLE_INDEX_WIDTH && final_y > MIDDLE_INDEX_HEIGHT {
        quadrants.1 += 1;
    }
    if final_x > MIDDLE_INDEX_WIDTH && final_y < MIDDLE_INDEX_HEIGHT {
        quadrants.2 += 1;
    }
    if final_x > MIDDLE_INDEX_WIDTH && final_y > MIDDLE_INDEX_HEIGHT {
        quadrants.3 += 1;
    }
}

pub fn task1() -> usize {
    let lines = crate::utils::read_file_to_lines("day14.in").unwrap();
    
    let mut quadrants = (0, 0, 0, 0);
    for line in &lines {
        simulate_robot(parse_line(line), &mut quadrants);
    }

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}
