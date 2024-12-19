use aoc_runner_derive::{aoc, aoc_generator};

use crate::{Direction, Grid, Point};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    #[default]
    Empty,
    Wall,
    Box,
    BigBoxLeft,
    BigBoxRight,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub tiles: Grid<Tile>,
    pub position: Point,
    pub instructions: Vec<Direction>,
    pub index: usize,
}

impl State {
    pub fn clone_but_wider(&self) -> State {
        let mut new_tiles = Grid::new(self.tiles.width * 2, self.tiles.height);

        for y in 0..self.tiles.height {
            for x in 0..self.tiles.width {
                let tile = self.tiles.get((x, y)).unwrap();

                let (left, right) = match tile {
                    Tile::Wall | Tile::Empty => (*tile, *tile),
                    Tile::Box | Tile::BigBoxLeft | Tile::BigBoxRight => {
                        (Tile::BigBoxLeft, Tile::BigBoxRight)
                    }
                };

                new_tiles.set((x * 2, y), left);
                new_tiles.set((x * 2 + 1, y), right);
            }
        }

        let new_position = (self.position.x * 2, self.position.y).into();

        State {
            tiles: new_tiles,
            position: new_position,
            instructions: self.instructions.clone(),
            index: self.index,
        }
    }

    fn can_move(&self, position: Point, direction: Direction) -> bool {
        let new_position = position + direction;

        match self.tiles.get(new_position) {
            Some(Tile::Wall) | None => false,
            Some(Tile::Empty) => true,
            Some(Tile::Box) => self.can_move(new_position, direction),

            // Big boxes always act as the left half to avoid duplication
            Some(Tile::BigBoxLeft) => match direction {
                Direction::Up | Direction::Down => {
                    self.can_move(new_position, direction)
                        && self.can_move(new_position + Direction::Right, direction)
                }
                Direction::Left => self.can_move(new_position, direction),
                Direction::Right => self.can_move(new_position + Direction::Right, direction),
            },
            Some(Tile::BigBoxRight) => self.can_move(position + Direction::Left, direction),
        }
    }

    fn push(&mut self, position: Point, direction: Direction) {
        // WARN: Assumes that it's safe to push!!

        let new_position = position + direction;

        match self.tiles.get(position) {
            // Walls and empty spaces don't move
            Some(Tile::Wall) | Some(Tile::Empty) | None => {}

            Some(Tile::Box) => {
                self.push(new_position, direction);
                self.tiles.set(new_position, Tile::Box);
                self.tiles.set(position, Tile::Empty);
            }

            // Big boxes always act as the left half to avoid duplication
            Some(Tile::BigBoxLeft) => match direction {
                Direction::Up | Direction::Down => {
                    self.push(new_position, direction);
                    self.push(new_position + Direction::Right, direction);
                    self.tiles.set(new_position, Tile::BigBoxLeft);
                    self.tiles
                        .set(new_position + Direction::Right, Tile::BigBoxRight);
                    self.tiles.set(position, Tile::Empty);
                    self.tiles.set(position + Direction::Right, Tile::Empty);
                }
                Direction::Left => {
                    self.push(new_position, direction);
                    self.tiles.set(new_position, Tile::BigBoxLeft);
                    self.tiles.set(position, Tile::BigBoxRight);
                    self.tiles.set(position + Direction::Right, Tile::Empty);
                }
                Direction::Right => {
                    self.push(new_position + Direction::Right, direction);
                    self.tiles
                        .set(new_position + Direction::Right, Tile::BigBoxRight);
                    self.tiles.set(new_position, Tile::BigBoxLeft);
                    self.tiles.set(position, Tile::Empty);
                }
            },
            Some(Tile::BigBoxRight) => {
                self.push(position + Direction::Left, direction);
            }
        }
    }

    fn score(&self) -> usize {
        self.tiles
            .iter_enumerate()
            .map(|(p, t)| match t {
                Tile::Box | Tile::BigBoxLeft => p.y as usize * 100 + p.x as usize,
                _ => 0,
            })
            .sum()
    }
}

impl Iterator for State {
    type Item = (Direction, Point);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.instructions.len() {
            return None;
        }

        let direction = self.instructions[self.index];
        self.index += 1;

        let new_position = self.position + direction;

        if self.can_move(self.position, direction) {
            self.push(new_position, direction);
            self.position = new_position;
        }

        Some((direction, self.position))
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for y in 0..self.tiles.height {
            for x in 0..self.tiles.width {
                let tile = self.tiles.get((x, y)).unwrap();

                let c = if self.position.x == x as i32 && self.position.y == y as i32 {
                    '@'
                } else {
                    match tile {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Box => 'O',
                        Tile::BigBoxLeft => '[',
                        Tile::BigBoxRight => ']',
                    }
                };

                s.push(c);
            }

            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

#[aoc_generator(day15)]
pub fn parse(input: &str) -> State {
    let (tile_data, instruction_data) = input.split_once("\n\n").unwrap();

    let tiles = Grid::read(tile_data, &|c| match c {
        '#' => Tile::Wall,
        '.' => Tile::Empty,
        'O' => Tile::Box,
        '@' => Tile::Empty,
        _ => panic!("unexpected character"),
    });

    let position_index = tile_data.chars().position(|c| c == '@').unwrap() as i32;
    let newline_width = tiles.width + 1;
    let position = (
        position_index % newline_width as i32,
        position_index / newline_width as i32,
    )
        .into();

    let instructions = instruction_data
        .chars()
        .filter_map(|c| match c {
            'v' => Some(Direction::Down),
            '^' => Some(Direction::Up),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect();

    State {
        tiles,
        position,
        instructions,
        index: 0,
    }
}

#[aoc(day15, part1, v1)]
fn part1_v1(input: &State) -> usize {
    let mut state = input.clone();

    for (_d, _p) in state.by_ref() {
        // println!("After: {_d:?} {_p:?}\n{state}");
    }
    // println!("Final:\n{}", state);

    state.score()
}

#[aoc(day15, part2, v1)]
fn part2_v1(input: &State) -> usize {
    let mut state = input.clone_but_wider();

    for (_d, _p) in state.by_ref() {
        // println!("After: {_d:?} {_p:?}\n{state}");
    }
    // println!("Final:\n{}", state);

    state.score()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_test;

    const SMALL_EXAMPLE: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const WIDER_SMALL_EXAMPLE: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    const EXAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    make_test!([part1_v1] => "day15.txt", 10092, 1552879);
    make_test!([part2_v1] => "day15.txt", 9021, 1561175);

    #[test]
    fn test_part1_v1_small_example() {
        let input = parse(SMALL_EXAMPLE);
        assert_eq!(part1_v1(&input).to_string(), "2028");
    }

    #[test]
    fn test_part2_v1_small_example() {
        let input = parse(SMALL_EXAMPLE);
        assert_eq!(part2_v1(&input).to_string(), "1751");
    }

    #[test]
    fn test_part2_v1_wider_small_example() {
        let input = parse(WIDER_SMALL_EXAMPLE);
        assert_eq!(part2_v1(&input).to_string(), "618");
    }
}
