use aoc_runner_derive::{aoc, aoc_generator};

use crate::{Grid, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Robot {
    pub position: Point,
    pub velocity: Point,
}

#[aoc_generator(day14)]
pub fn parse(input: &str) -> (usize, usize, Vec<Robot>) {
    let mut width = 101;
    let mut height = 103;

    let mut robots = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("#") {
            let (w, h) = line[2..].split_once("x").unwrap();
            width = w.parse().unwrap();
            height = h.parse().unwrap();
            continue;
        }

        let (point, velocity) = line.split_once(" ").unwrap();

        let (px, py) = point[2..].split_once(",").unwrap();
        let (vx, vy) = velocity[2..].split_once(",").unwrap();

        robots.push(Robot {
            position: (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap()).into(),
            velocity: (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap()).into(),
        })
    }

    (width, height, robots)
}

#[allow(dead_code)]
fn print_state(width: usize, height: usize, robots: &[Robot]) {
    let mut grid: Grid<usize> = Grid::new(width, height);

    for robot in robots.iter() {
        *grid.get_mut(robot.position).unwrap() += 1;
    }

    println! {"{}", grid.to_string(&|&x| if x == 0 { ".".to_string() } else { x.to_string() })};
}

#[aoc(day14, part1, v1)]
fn part1_v1((width, height, input): &(usize, usize, Vec<Robot>)) -> usize {
    let mut robots = input.clone();

    for _i in 0..100 {
        for robot in robots.iter_mut() {
            robot.position += robot.velocity;
            robot.position.x = robot.position.x.rem_euclid(*width as i32);
            robot.position.y = robot.position.y.rem_euclid(*height as i32);
        }
    }

    let mut quadrant_scores = [0; 4];

    let half_width_left = *width as i32 / 2;
    let half_height_left = *height as i32 / 2;

    // The bottom/right quadrants do not include the middle row if the height/width is odd
    let half_height_right = if height % 2 == 1 {
        half_height_left + 1
    } else {
        half_height_left
    };

    let half_width_right = if width % 2 == 1 {
        half_width_left + 1
    } else {
        half_width_left
    };

    for robot in robots.iter() {
        if robot.position.x < half_width_left && robot.position.y < half_height_left {
            quadrant_scores[0] += 1;
        } else if robot.position.x >= half_width_right && robot.position.y < half_height_left {
            quadrant_scores[1] += 1;
        } else if robot.position.x < half_width_left && robot.position.y >= half_height_right {
            quadrant_scores[2] += 1;
        } else if robot.position.x >= half_width_right && robot.position.y >= half_height_right {
            quadrant_scores[3] += 1;
        }
        // Any other robots are on a line
    }

    quadrant_scores.iter().product::<usize>()
}

#[aoc(day14, part2, v1)]
fn part2_v1((width, height, input): &(usize, usize, Vec<Robot>)) -> usize {
    if *width != 101 || *height != 103 {
        return 0; // this doesn't work for the example case
    }

    let mut robots = input.clone();

    // Advance each robot until the image magically appears
    let mut timer = 0;
    loop {
        timer += 1;

        for robot in robots.iter_mut() {
            robot.position += robot.velocity;
            robot.position.x = robot.position.x.rem_euclid(*width as i32);
            robot.position.y = robot.position.y.rem_euclid(*height as i32);
        }

        // Target image is 31x33 with a 1x border
        // Let's try just detecting the border
        let mut grid: Grid<bool> = Grid::new(*width, *height);
        for robot in robots.iter() {
            *grid.get_mut(robot.position).unwrap() = true;
        }

        let mut border_found = true;

        'border_patrol: for start_x in 0..*width {
            'next_point: for start_y in 0..*height {
                for xd in 0..31 {
                    if grid.get((start_x + xd, start_y)) != Some(&true) {
                        border_found = false;
                        continue 'next_point;
                    }
                    if grid.get((start_x + xd, start_y + 32)) != Some(&true) {
                        border_found = false;
                        continue 'next_point;
                    }
                }

                for yd in 0..33 {
                    if grid.get((start_x, start_y + yd)) != Some(&true) {
                        border_found = false;
                        continue 'next_point;
                    }
                    if grid.get((start_x + 30, start_y + yd)) != Some(&true) {
                        border_found = false;
                        continue 'next_point;
                    }
                }

                border_found = true;
                break 'border_patrol;
            }
        }

        if border_found {
            return timer;
        }

        assert!(
            timer < 100_000,
            "Did not find symmetric image in 100_000 iterations"
        );
    }
}

#[aoc(day14, part2, v2)]
fn part2_v2((width, height, input): &(usize, usize, Vec<Robot>)) -> usize {
    if *width != 101 || *height != 103 {
        return 0; // this doesn't work for the example case
    }

    let mut robots = input.clone();
    let mut hline_start = None;
    let mut vline_start = None;

    // Advance each robot until the image magically appears
    let mut timer = 0;
    loop {
        timer += 1;

        for robot in robots.iter_mut() {
            robot.position += robot.velocity;
            robot.position.x = robot.position.x.rem_euclid(*width as i32);
            robot.position.y = robot.position.y.rem_euclid(*height as i32);
        }

        // Check for unusually many 'busy' horizontal lines
        if hline_start.is_none() {
            let mut hline_counts: Vec<usize> = vec![0; *height];
            for robot in robots.iter() {
                hline_counts[robot.position.y as usize] += 1;
            }

            if hline_counts.iter().filter(|v| **v > 20).count() > 3 {
                hline_start = Some(timer);
            }
        }

        // Check for unusually many 'busy' vertical lines
        if vline_start.is_none() {
            let mut vline_counts: Vec<usize> = vec![0; *width];
            for robot in robots.iter() {
                vline_counts[robot.position.x as usize] += 1;
            }

            if vline_counts.iter().filter(|v| **v > 20).count() > 3 {
                vline_start = Some(timer);
            }
        }

        // If we have both, we have an answer
        // I'm still not sure why the cycles can be off by ±1
        if hline_start.is_some() && vline_start.is_some() {
            // Solve using the Chinese remainder theorem
            let h_offset = hline_start.unwrap() % *height;
            let v_offset = vline_start.unwrap() % *width;

            let mut h_timer = h_offset;
            let mut v_timer = v_offset;

            loop {
                if h_timer == v_timer {
                    return h_timer;
                }

                if h_timer < v_timer {
                    h_timer += *height;
                } else {
                    v_timer += *width;
                }
            }
        }

        assert!(
            timer < 10_000,
            "Did not find symmetric image in 100_000 iterations"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const EXAMPLE: &str = "\
# 11x7
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    make_test!([part1_v1] => "day14.txt", 12, 219150360);

    make_test!([part2_v1, part2_v2] => "day14.txt", 0, 8053);
}

