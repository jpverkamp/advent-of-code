use fxhash::FxHashSet;

use grid::Grid;
use point::Point;

pub fn read(input: &str) -> (FxHashSet<Point>, Point) {
    let walls = Grid::read(input, |c| if c == '#' { Some(true) } else { None });
    let walls = walls.iter().map(|(p, _)| *p).collect::<FxHashSet<_>>();

    let start = Grid::read(input, |c| if c == 'S' { Some(true) } else { None });
    let start = *start.iter().next().unwrap().0;

    (walls, start)
}
