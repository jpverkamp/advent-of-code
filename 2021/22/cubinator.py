import logging
import typer
import re

from collections import *
from dataclasses import dataclass
from typing import *

app = typer.Typer()


def disableable_cache(x): return x


@dataclass(frozen=True, order=True)
class Point:
    x: int
    y: int
    z: int


@dataclass(frozen=True)
class Cube:
    min: Point
    max: Point

    def __repr__(self):
        return f'Cube({len(self)})@[{self.min.x}..{self.max.x}, {self.min.y}..{self.max.y}, {self.min.z}..{self.max.z}]'

    @disableable_cache
    def __len__(self) -> int:
        return (
            (self.max.x - self.min.x)
            * (self.max.y - self.min.y)
            * (self.max.z - self.min.z)
        )

    def __iter__(self) -> Generator[Point, None, None]:
        for x in range(self.min.x, self.max.x):
            for y in range(self.min.y, self.max.y):
                for z in range(self.min.z, self.max.z):
                    yield Point(x, y, z)

    @disableable_cache
    def __contains__(self, other: Union[Point, 'Cube']) -> bool:
        '''Is the cube/point other entirely contained in self.'''

        if isinstance(other, Point):
            return (
                self.min.x <= other.x <= self.max.x
                and self.min.y <= other.y <= self.max.y
                and self.min.z <= other.z <= self.max.z
            )
        elif isinstance(other, Cube):
            return other.min in self and other.max in self

    @disableable_cache
    def overlaps(self, other: 'Cube') -> bool:
        '''
        Does the cube overlap with self at all?
        
        Note: This includes if the cubes are just touching since edges are inclusive.
        '''

        return (
            self.min in other
            or self.max in other
            or other.min in other
            or other.max in other
            or self in other
            or other in self
        )

    @disableable_cache
    def __segment(self, other: 'Cube') -> List['Cube']:
        xs = list(sorted([self.min.x, self.max.x, other.min.x, other.max.x]))
        ys = list(sorted([self.min.y, self.max.y, other.min.y, other.max.y]))
        zs = list(sorted([self.min.z, self.max.z, other.min.z, other.max.z]))

        segments = []

        for x1, x2 in zip(xs, xs[1:]):
            for y1, y2 in zip(ys, ys[1:]):
                for z1, z2 in zip(zs, zs[1:]):
                    new_segment = Cube(Point(x1, y1, z1), Point(x2, y2, z2))

                    if new_segment in segments:
                        continue

                    segments.append(new_segment)

        return segments

    @disableable_cache
    def join(self, other: 'Cube') -> Optional['Cube']:
        '''If two cubes can be perfectly joined, return that.'''

        # One cube contains the other
        if self in other:
            return other
        elif other in self:
            return self

        # The x/y/z edges match perfectly
        x_match = self.min.x == other.min.x and self.max.x == other.max.x
        y_match = self.min.y == other.min.y and self.max.y == other.max.y
        z_match = self.min.z == other.min.z and self.max.z == other.max.z

        # The last dimension is contained within the other cube
        x_overlap = (
            (other.min.x <= self.min.x <= other.max.x)
            or (other.min.x <= self.max.x <= other.max.x)
            or (self.min.x <= other.min.x <= self.min.x)
            or (self.min.x <= other.max.x <= self.min.x)
        )

        y_overlap = (
            (other.min.y <= self.min.y <= other.max.y)
            or (other.min.y <= self.max.y <= other.max.y)
            or (self.min.y <= other.min.y <= self.min.y)
            or (self.min.y <= other.max.y <= self.min.y)
        )

        z_overlap = (
            (other.min.z <= self.min.z <= other.max.z)
            or (other.min.z <= self.max.z <= other.max.z)
            or (self.min.z <= other.min.z <= self.min.z)
            or (self.min.z <= other.max.z <= self.min.z)
        )

        # If we have exactly two matches and an overlap, we can combine
        if (
            (x_overlap and y_match and z_match)
            or (x_match and y_overlap and z_match)
            or (x_match and y_match and z_overlap)
        ):
            result = Cube(min(self.min, other.min), max(self.max, other.max))
            return result

        return None

    @disableable_cache
    @staticmethod
    def compress(cubes: List['Cube']) -> List['Cube']:
        '''Take a list of cubes and join as many as we can.'''

        logging.debug(f'> compress({cubes}')

        cubes = list(cubes)

        def find_one_join():
            for i, c1 in enumerate(cubes):
                for j, c2 in enumerate(cubes):
                    if j <= i:
                        continue

                    if c := c1.join(c2):
                        return i, j, c

        while True:
            if result := find_one_join():
                i, j, c = result

                del cubes[j]
                del cubes[i]
                cubes.append(c)
            else:
                break

        return cubes

    @disableable_cache
    def __and__(self, other: 'Cube') -> List['Cube']:
        '''Calculate the list of cubes making up the intersection of self and other.'''

        logging.debug(f'> {self} & {other}')

        # One cube is entirely inside of the other
        if self in other:
            return [self]
        elif other in self:
            return [other]

        # No overlap at all
        elif not self.overlaps(other):
            return []

        # Finally, only segments that are in both
        else:
            return Cube.compress([
                segment
                for segment in self.__segment(other)
                if segment in self and segment in other
            ])

    @disableable_cache
    def __or__(self, other: 'Cube') -> List['Cube']:
        '''Calculate the list of cubes making up the union of self and other.'''

        logging.debug(f'> {self} | {other}')

        # One cube is entirely inside the other
        if self in other:
            return [other]
        elif other in self:
            return [self]

        # No overlap at all
        elif not self.overlaps(other):
            return [self, other]

        # Otherwise, split into segments and return all of them
        else:
            return Cube.compress([
                segment
                for segment in self.__segment(other)
                if segment in self or segment in other
            ])

    @disableable_cache
    def __add__(self, other: 'Cube') -> List['Cube']:
        '''Adding is the same as intersection.'''

        logging.debug(f'> {self} + {other}')

        return self | other

    @disableable_cache
    def __sub__(self, other: 'Cube') -> List['Cube']:
        '''Calculate the list of cubes resulting of removing other from self.'''

        logging.debug(f'> {self} - {other}')

        # Subtract the entire thing
        if self in other:
            return []

        # No overlap at all
        elif not self.overlaps(other):
            return [self]

        return Cube.compress([
            segment
            for segment in self.__segment(other)
            if segment in self and segment not in other
        ])


def read(file: TextIO) -> Generator[Tuple[bool, Cube], None, None]:
    for line in file:
        m = re.match(r'(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)', line)
        if m:
            mode, x1, x2, y1, y2, z1, z2 = m.groups()
            cube = Cube(
                Point(int(x1), int(y1), int(z1)),
                Point(int(x2) + 1, int(y2) + 1, int(z2) + 1)
            )

            yield mode == 'on', cube


@app.command()
def main(file: typer.FileText, limit: bool = False):
    cubes: List[Cube] = []

    for i, (turn_on, cube) in enumerate(read(file), 1):
        logging.info(f'{i:04d}: {turn_on=} {cube=}')

        # Turning on cubes, don't turn on anything that is already on
        if turn_on:
            to_turn_on = [cube]

            for old_cube in cubes:
                to_turn_on = [
                    remaining_cube
                    for to_turn_on_cube in to_turn_on
                    for remaining_cube in to_turn_on_cube - old_cube
                ]

            cubes += to_turn_on

        # Turning off cubes, turn off anything that should be off
        else:
            cubes = [
                reduced_cube
                for current_cube in cubes
                for reduced_cube in current_cube - cube
            ]

        # Re-compress at the end of each cycle
        cubes = Cube.compress(cubes)
        logging.info(f'      {len(cubes)=}, {sum(len(cube) for cube in cubes)=}\n')

        # Remove all regions outside of -50..50
        # This is silly, because we'll need to keep them in part 2
        # But it's faster at least
        if limit:
            cubes = Cube.compress([
                reduced
                for cube in cubes
                for reduced in cube & Cube(Point(-50, -50, -50), Point(51, 51, 51))
            ])

    print(sum(len(cube) for cube in cubes))


@app.command()
def part1(file: typer.FileText):
    main(file, True)


@app.command()
def part2(file: typer.FileText):
    main(file, False)


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
