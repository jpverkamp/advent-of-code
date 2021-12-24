import itertools
import logging
import typer

from collections import *
from dataclasses import dataclass
from typing import *

app = typer.Typer()

BEACON_OVERLAPPINGNESS = 12


def disableable_cache(x): return x


@dataclass(frozen=True)
class Point:
    x: int
    y: int
    z: int

    def __repr__(self):
        return f'{{{self.x}, {self.y}, {self.z}}}'

    @staticmethod
    def all_waggles() -> Generator['Point', None, None]:
        '''Generate all waggle parameters (see Point.waggle)'''

        for i, j, k in itertools.permutations((1, 2, 3)):
            for ix in (-i, i):
                for jx in (-j, j):
                    for kx in (-k, k):
                        yield Point(ix, jx, kx)

    def waggle(self, w: 'Point') -> 'Point':
        '''
        Return a new point reflected/reordered by the given coordinates.

        Each of w's x,y,z should be +- 1,2,3 and each of 1,2,3 should be used exactly once.

        reflect(3, 2, -1) should return Point(z, y, -x) for example.
        '''

        d = (-1, self.x, self.y, self.z)

        return Point(
            d[abs(w.x)] * (-1 if w.x < 0 else 1),
            d[abs(w.y)] * (-1 if w.y < 0 else 1),
            d[abs(w.z)] * (-1 if w.z < 0 else 1),
        )

    def __add__(self, other: 'Point') -> 'Point':
        '''Return the sum of two points.'''

        return Point(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other: 'Point') -> 'Point':
        '''Return the difference of two points, obv.'''

        return Point(self.x - other.x, self.y - other.y, self.z - other.z)


@dataclass(frozen=True)
class Scanner:
    name: str
    points: FrozenSet[Point]

    @staticmethod
    def read(file: TextIO) -> Optional['Scanner']:
        '''Read a Scanner from a filelike object'''

        if not (name := file.readline().strip('- \n')):
            return None

        points = set()
        while line := file.readline().strip():
            points.add(Point(*[int(v) for v in line.split(',')]))

        return Scanner(name, frozenset(points))

    def __or__(self, other: 'Scanner') -> Optional[Tuple[Point, Point, Set[Point]]]:
        '''Given another scanner, try to find the overlapping points.'''

        # Try every waggle of their scanners, assume I'm always right
        for their_waggle in Point.all_waggles():
            their_waggled_points = {
                p.waggle(their_waggle)
                for p in other.points
            }

            logging.debug(f'Comparing {self.points=} and {their_waggled_points=}')

            # Choose where we think the 'other' scanner is from our perspective
            for my_zero in self.points:
                my_zeroed_points = {p - my_zero for p in self.points}

                for their_waggled_zero in their_waggled_points:
                    their_zeroed_points = {p - their_waggled_zero for p in their_waggled_points}

                    # Try to subtract that from all of our points
                    # If we have enough matches, that means we know their scanner from our point of view
                    matches = my_zeroed_points & their_zeroed_points

                    if len(matches) >= BEACON_OVERLAPPINGNESS:
                        return (their_waggle, my_zero - their_waggled_zero, {p + my_zero for p in matches})

        return None

    def __repr__(self):
        return f'@{{{self.name}}}'


def do_the_actual_work(scanners: List[Scanner]) -> Mapping[Tuple[Scanner, Scanner], Tuple[Point, Point, Set[Point]]]:

    logging.info('=== FINDING INITIAL OFFSETS ===')
    offsets: MutableMapping[Tuple[Scanner, Scanner], Tuple[Point, Point, Set[Point]]] = {}

    for s0 in scanners:
        for s1 in scanners:
            logging.info(f'Finding the offset from {s0=} to {s1=}')

            if (s1, s0) in offsets:
                offsets[s0, s1] = offsets[s1, s0]

            if result := (s0 | s1):
                offsets[s0, s1] = result

    # Fill in the entire chart
    logging.info('=== EXPANDING CHART ===')
    s0 = scanners[0]

    # If we don't have a path from s0 to s1, try to go s0 -> svia -> s1
    updating = True
    while updating:
        logging.info('Working on expanding iteration')
        updating = False

        for s1 in scanners[1:]:
            if (s0, s1) in offsets:
                continue

            for svia in scanners[1:]:
                if s0 == svia or svia == s1:
                    continue

                if not ((s0, svia) in offsets and (svia, s1) in offsets):
                    continue

                logging.info(f'Building a new path from {s0} via {svia} to {s1}')

                waggle1, offset1, _ = offsets[s0, svia]
                waggle2, offset2, points2 = offsets[svia, s1]

                new_waggle = waggle2.waggle(waggle1)
                new_offset = offset1 + offset2.waggle(waggle1)

                new_points = {
                    offset1 + p.waggle(waggle1)
                    for p in points2
                }

                offsets[s0, s1] = new_waggle, new_offset, new_points
                updating = True

    return offsets


@app.command()
def part1(file: typer.FileText):

    scanners = []
    while s := Scanner.read(file):
        scanners.append(s)
    s0 = scanners[0]

    offsets = do_the_actual_work(scanners)

    # Finally, calculate all of the beacon points
    logging.info('=== FINDING BEACONS ===')

    all_beacons = set()
    for scanner in scanners:
        waggle, offset, _ = offsets[s0, scanner]
        logging.info(f'Adding beacons from {scanner} at {offset} (with {waggle=})')

        all_beacons |= {
            offset + p.waggle(waggle)
            for p in scanner.points
        }

    print(len(all_beacons))


@app.command()
def part2(file: typer.FileText):

    scanners = []
    while s := Scanner.read(file):
        scanners.append(s)
    s0 = scanners[0]

    offsets = do_the_actual_work(scanners)

    # Finally, calculate all of the beacon points
    logging.info('=== FINDING SCANNERS ===')

    locations = set()
    for scanner in scanners:
        _, offset, _ = offsets[s0, scanner]
        locations.add(offset)

    print(max(
        abs(l1.x - l2.x) + abs(l1.y - l2.y) + abs(l1.z - l2.z)
        for l1 in locations
        for l2 in locations
    ))


@app.callback()
def enableFlags(cache: bool = False, debug: bool = False):
    if debug:
        import coloredlogs  # type: ignore
        coloredlogs.install(level=logging.INFO)

    if cache:
        import functools
        global disableable_cache
        disableable_cache = functools.cache


if __name__ == '__main__':
    app()
