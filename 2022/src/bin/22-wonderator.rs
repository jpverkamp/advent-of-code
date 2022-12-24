use aoc::*;
use image::{ImageBuffer, RgbImage};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env,
    path::Path,
};

#[derive(Debug)]
struct Map {
    start: Point,
    width: usize,
    height: usize,
    walls: HashSet<Point>,
    floors: HashSet<Point>,
    path_data: Vec<(Point, char)>,
}

impl<I> From<&mut I> for Map
where
    I: Iterator<Item = String>,
{
    fn from(iter: &mut I) -> Self {
        let mut walls = HashSet::new();
        let mut floors = HashSet::new();
        let mut width = 0;
        let mut height = 0;
        let mut start: Option<Point> = None;

        iter.take_while(|line| !line.is_empty())
            .enumerate()
            .for_each(|(y, line)| {
                height = height.max(y + 1);
                line.chars().enumerate().for_each(|(x, c)| match c {
                    '#' => {
                        width = width.max(x + 1);
                        walls.insert(Point::new(x as isize + 1, y as isize + 1));
                    }
                    '.' => {
                        if start.is_none()
                            || y as isize + 1 < start.unwrap().y
                            || (y as isize + 1 <= start.unwrap().y
                                && x as isize + 1 < start.unwrap().x)
                        {
                            start = Some(Point::new(x as isize + 1, y as isize + 1));
                        }

                        width = width.max(x + 1);
                        floors.insert(Point::new(x as isize + 1, y as isize + 1));
                    }
                    _ => {}
                })
            });

        Map {
            start: start.unwrap(),
            width,
            height,
            walls,
            floors,
            path_data: vec![(start.unwrap(), '>')],
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        output.push_str(
            format!(
                "Map<width: {}, height: {}, start: {:?}, data:\n",
                self.width, self.height, self.start
            )
            .as_str(),
        );
        for y in 1..=self.height {
            for x in 1..=self.width {
                let p = Point::new(x as isize, y as isize);
                if self.walls.contains(&p) {
                    output.push('#');
                } else if self.floors.contains(&p) {
                    if let Some((_, facing)) = self.path_data.iter().find(|(pp, _)| p == *pp) {
                        output.push(*facing);
                    } else {
                        output.push('.');
                    }
                } else {
                    output.push(' ');
                }
            }
            output.push('\n');
        }
        output.push('>');

        write!(f, "{output}")
    }
}

impl Map {
    fn calculate_move(
        &mut self,
        location: Point,
        facing: Facing,
        distance: usize,
        wrap_mode: &WrapMode,
    ) -> (Point, Facing) {
        use Facing::*;

        let mut current = (location, facing);
        self.path_data.push((current.0, current.1.char()));

        for _i in 0..distance {
            let mut next = (current.0 + current.1.delta(), current.1);

            // If we run into a wall, just stop moving
            if self.walls.contains(&next.0) {
                break;
            }

            // If we run off the map, wrap
            if !self.floors.contains(&next.0) {
                // Step back onto the floor
                next = current;

                // Different wrapping options depending on the mode
                match wrap_mode {
                    // Loop is defined as walking the opposite way until you hit an empty space
                    WrapMode::Loop => {
                        // Skip back across the walls and floors this time
                        while self.floors.contains(&next.0) || self.walls.contains(&next.0) {
                            next.0 = next.0 - current.1.delta();
                        }

                        // Step back once off empty space
                        next.0 = next.0 + current.1.delta();
                    }
                    // Cube is defined as wrapping onto the next face of a cube
                    WrapMode::Cube(width, adjacencies) => {
                        // Determine the index of the face
                        let current_face = Point::new(
                            (next.0.x - 1) / (*width as isize),
                            (next.0.y - 1) / (*width as isize),
                        );

                        // Figure out how far we are side to side on that face
                        // Offset is from the 'left' according to facing
                        let current_offset = match facing {
                            North => (next.0.x - 1) % (*width as isize),
                            South => *width as isize - (next.0.x - 1) % (*width as isize) - 1,
                            East => (next.0.y - 1) % (*width as isize),
                            West => *width as isize - (next.0.y - 1) % (*width as isize) - 1,
                        };

                        // Determine the next face and facing
                        let (next_face, next_facing) =
                            adjacencies.get(&(current_face, facing)).expect(
                                format!("unknown adjacency for {current_face:?} facing {facing:?}")
                                    .as_str(),
                            );

                        next = (
                            Point::new(
                                1 + next_face.x * (*width as isize)
                                    + match next_facing {
                                        North => current_offset,
                                        South => *width as isize - current_offset - 1,
                                        East => 0,
                                        West => *width as isize - 1,
                                    },
                                1 + next_face.y * (*width as isize)
                                    + match next_facing {
                                        North => *width as isize - 1,
                                        South => 0,
                                        East => current_offset,
                                        West => *width as isize - current_offset - 1,
                                    },
                            ),
                            *next_facing,
                        );
                    }
                }

                // If we have a wall after wrapping, don't move
                if self.walls.contains(&next.0) {
                    break;
                }
            }

            // If we made it out of both checks, we have the new current point
            self.path_data.push((current.0, current.1.char()));
            current = next;
        }

        current
    }

    fn render(&self) -> RgbImage {
        ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
            let p = Point::new(x as isize, y as isize);
            if self.walls.contains(&p) {
                image::Rgb([127, 127, 127])
            } else if self.floors.contains(&p) {
                if let Some((index, (_, facing))) = self
                    .path_data
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, (pp, _))| p == *pp)
                {
                    let c = if index > 223 { 32 } else { (255 - index) as u8 };

                    match facing {
                        '^' => image::Rgb([c, 15, 15]),
                        'v' => image::Rgb([15, c, 15]),
                        '<' => image::Rgb([15, 15, c]),
                        '>' => image::Rgb([c, c, 15]),
                        _ => panic!("unknown facing char {c}"),
                    }
                } else {
                    image::Rgb([15, 15, 15])
                }
            } else {
                image::Rgb([0, 0, 0])
            }
        })
    }
}

#[derive(Debug)]
enum WrapMode {
    Loop,
    Cube(usize, HashMap<(Point, Facing), (Point, Facing)>),
}

#[derive(Debug)]
struct Moves {
    data: Vec<(usize, char)>,
}

impl From<String> for Moves {
    fn from(line: String) -> Self {
        let iter = &mut line.chars().peekable();
        let mut data = Vec::new();

        while !iter.peek().is_none() {
            // Distance is a positive integer, might be more than one digit
            let distance = iter
                .peeking_take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<usize>()
                .expect("must be parsable as a number");

            // Parse L or R as a turn, add an L at the end if we end with a number
            let turn = if iter.peek().is_some() {
                iter.next().unwrap()
            } else {
                'X'
            };

            data.push((distance, turn));
        }

        Moves { data }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Default for Facing {
    fn default() -> Self {
        Facing::East
    }
}

impl Facing {
    fn turn(self, turn: char) -> Self {
        use Facing::*;

        match (self, turn) {
            (North, 'L') => West,
            (North, 'R') => East,
            (South, 'L') => East,
            (South, 'R') => West,

            (East, 'L') => North,
            (East, 'R') => South,
            (West, 'L') => South,
            (West, 'R') => North,

            (_, 'X') => self,

            _ => panic!("don't know how to turn {turn} from {self:?}"),
        }
    }

    fn opposite(self) -> Self {
        use Facing::*;

        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

    fn delta(self) -> Point {
        use Facing::*;

        match self {
            North => Point::new(0, -1),
            South => Point::new(0, 1),
            East => Point::new(1, 0),
            West => Point::new(-1, 0),
        }
    }

    fn char(&self) -> char {
        use Facing::*;

        match self {
            North => '^',
            South => 'v',
            East => '>',
            West => '<',
        }
    }

    fn value(&self) -> isize {
        use Facing::*;

        match self {
            East => 0,
            South => 1,
            West => 2,
            North => 3,
        }
    }
}

fn part1(filename: &Path) -> String {
    let mut iter = &mut iter_lines(filename);
    let mut map = Map::from(&mut iter);
    let moves = Moves::from(iter.next().expect("must have moves"));

    let mut location = map.start.clone();
    let mut facing = Facing::East;

    for (distance, turn) in moves.data.into_iter() {
        if cfg!(debug_assertions) {
            println!("next move is {distance} and then turn {turn}");
        }

        (location, _) = map.calculate_move(location, facing, distance, &WrapMode::Loop);
        facing = facing.turn(turn);

        if cfg!(debug_assertions) {
            println!("moved to {location:?}, {facing:?}");
            println!("{map}\n");
        }
    }

    let password = 1000 * location.y + 4 * location.x + facing.value();
    if cfg!(debug_assertions) {
        println!(
            "1000 * {} + 4 * {} + {} = {}",
            location.y,
            location.x,
            facing.value(),
            password
        );
    }

    password.to_string()
}

fn part2(filename: &Path) -> String {
    let mut iter = &mut iter_lines(filename);
    let mut map = Map::from(&mut iter);
    let moves = Moves::from(iter.next().expect("must have moves"));

    let mut location = map.start.clone();
    let mut facing = Facing::East;

    use Facing::*;

    let test_mode = filename.to_str().unwrap().contains("test");

    let size = if test_mode { 4 } else { 50 };
    let adjacency_map: HashMap<(Point, Facing), (Point, Facing)> = (if test_mode {
        let faces = [
            Point::new(2, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(2, 2),
            Point::new(3, 2),
        ];

        // Hand calculated for the test map
        // TODO: Can this be automated?
        vec![
            ((faces[1 - 1], West), (faces[3 - 1], South)),
            ((faces[1 - 1], North), (faces[2 - 1], South)),
            ((faces[1 - 1], East), (faces[6 - 1], West)),
            ((faces[2 - 1], North), (faces[1 - 1], South)),
            ((faces[2 - 1], West), (faces[6 - 1], North)),
            ((faces[2 - 1], South), (faces[5 - 1], North)),
            ((faces[3 - 1], North), (faces[1 - 1], East)),
            ((faces[3 - 1], South), (faces[5 - 1], East)),
            ((faces[4 - 1], East), (faces[6 - 1], South)),
            ((faces[5 - 1], West), (faces[3 - 1], North)),
            ((faces[5 - 1], South), (faces[2 - 1], North)),
            ((faces[6 - 1], North), (faces[4 - 1], West)),
            ((faces[6 - 1], East), (faces[1 - 1], West)),
            ((faces[6 - 1], South), (faces[2 - 1], East)),
        ]
    } else {
        let faces = [
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(1, 1),
            Point::new(0, 2),
            Point::new(1, 2),
            Point::new(0, 3),
        ];

        // Hand calculated for my map
        // TODO: Can this be automated?
        vec![
            ((faces[1 - 1], North), (faces[6 - 1], East)),
            ((faces[1 - 1], West), (faces[4 - 1], East)),
            ((faces[2 - 1], North), (faces[6 - 1], North)),
            ((faces[2 - 1], East), (faces[5 - 1], West)),
            ((faces[2 - 1], South), (faces[3 - 1], West)),
            ((faces[3 - 1], West), (faces[4 - 1], South)),
            ((faces[3 - 1], East), (faces[2 - 1], North)),
            ((faces[4 - 1], North), (faces[3 - 1], East)),
            ((faces[4 - 1], West), (faces[1 - 1], East)),
            ((faces[5 - 1], East), (faces[2 - 1], West)),
            ((faces[5 - 1], South), (faces[6 - 1], West)),
            ((faces[6 - 1], West), (faces[1 - 1], South)),
            ((faces[6 - 1], East), (faces[5 - 1], North)),
            ((faces[6 - 1], South), (faces[2 - 1], South)),
        ]
    })
    .into_iter()
    .collect();

    for (pf1, pf2) in adjacency_map.iter() {
        let pf1p = (pf1.0, pf1.1.opposite());
        let pf2p = (pf2.0, pf2.1.opposite());

        if !adjacency_map.contains_key(&pf2p) {
            panic!("Expecing {pf2p:?} in adjacency_map to match {pf2:?}");
        }

        if adjacency_map[&pf2p] != pf1p {
            panic!(
                "Expecing value of {pf2p:?} to be {pf1p:?}, got {:?}",
                adjacency_map[&pf2p]
            );
        }
    }

    let wrap_mode = WrapMode::Cube(size, adjacency_map);

    let move_count = moves.data.len();
    for (frame, (distance, turn)) in moves.data.into_iter().enumerate() {
        if cfg!(debug_assertions) {
            println!("next move is {distance} and then turn {turn}");
        }

        (location, facing) = map.calculate_move(location, facing, distance, &wrap_mode);
        facing = facing.turn(turn);

        if cfg!(debug_assertions) {
            println!("moved to {location:?}, {facing:?}");
            println!("{map}\n");
        }

        if env::var("AOC22_RENDER").is_ok() {
            println!("Rendering [{frame}/{move_count}]");
            map.render()
                .save(format!("{:08}.png", frame))
                .expect("failed to save frame");
        }
    }

    let password = 1000 * location.y + 4 * location.x + facing.value();
    if cfg!(debug_assertions) {
        println!(
            "1000 * {} + 4 * {} + {} = {}",
            location.y,
            location.x,
            facing.value(),
            password
        );
    }

    if env::var("AOC22_RENDER").is_ok() {
        println!("Rendering mp4");

        use std::process::Command;

        let commands = vec![
            "ffmpeg -y -framerate 240 -i %08d.png -vf scale=iw*4:ih*4:flags=neighbor -c:v libx264 -r 30 aoc22.raw.mp4",
            "find . -name '*.png' | xargs rm",
            "ffmpeg -y -i aoc22.raw.mp4 -c:v libx264 -preset slow -crf 20 -vf format=yuv420p -movflags +faststart aoc22.mp4",
            "rm aoc22.raw.mp4",
        ];

        for cmd in commands.into_iter() {
            println!("$ {}", cmd);
            let mut child = Command::new("bash")
                .arg("-c")
                .arg(cmd)
                .spawn()
                .expect("command failed");
            child.wait().expect("process didn't finish");
        }
    }

    password.to_string()
}

fn main() {
    aoc_main(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use aoc::aoc_test;

    #[test]
    fn test1() {
        aoc_test("22", part1, "196134")
    }

    #[test]
    fn test2() {
        aoc_test("22", part2, "146011")
    }
}
