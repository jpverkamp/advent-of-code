import itertools
import logging
import typer
import re

from collections import *
from dataclasses import dataclass
from typing import *
from enum import Enum

app = typer.Typer()


def disableable_cache(x): return x


@dataclass(frozen=True)
class Point:
    '''Represents a point in 2-space, positive y is up'''

    x: int
    y: int


@dataclass(frozen=True)
class Rect:
    '''Represents a rectangle in 2-space, positive Y is up'''

    position: Point
    width: int
    height: int

    @staticmethod
    def from_file(file: TextIO):
        m = re.match(
            r'target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)',
            file.readline()
        )

        if m:
            x1, x2, y1, y2 = m.groups()
            return Rect(
                Point(int(x1), int(y1)),
                int(x2) - int(x1),
                int(y2) - int(y1)
            )

    def __contains__(self, p: Point):
        return (
            self.position.x <= p.x <= self.position.x + self.width
            and self.position.y <= p.y <= self.position.y + self.height
        )


@dataclass(frozen=True)
class Probe:
    '''Represents a probe with initial position and velocity.'''

    position: Point
    velocity: Point

    def update(self) -> 'Probe':
        return Probe(
            position=Point(
                self.position.x + self.velocity.x,
                self.position.y + self.velocity.y
            ),
            velocity=Point(
                self.velocity.x + (1 if self.velocity.x < 0 else -1 if self.velocity.x > 0 else 0),
                self.velocity.y - 1
            )
        )

    def impacts(self, target: Rect) -> Optional[int]:
        '''
        Tests if the given probe hits the given target.

        If yes: return the maximum height reached (the coolness factor)
        If no: return None
        '''

        current = self
        coolness = self.position.y

        while current.position.y >= target.position.y:
            if current.position in target:
                return coolness

            current = current.update()
            coolness = max(coolness, current.position.y)

        return None


def all_impacts(target) -> Generator[Tuple[Probe, int], None, None]:
    logging.info(f'all_impacts({target=}), starting')

    origin = Point(0, 0)
    phase = 0

    for offset in itertools.count(1):
        logging.info(f'all_impacts({target=}), {offset=}, {phase=}')

        at_least_one_impact = False

        for xd in range(offset+1):
            for yd in range(-offset, offset+1):
                if abs(xd) + abs(yd) != offset:
                    continue

                probe = Probe(origin, Point(xd, yd))
                coolness = probe.impacts(target)

                if coolness is not None:
                    logging.info(f'all_impacts({target=}), Hit! {xd=}, {yd=} -> {coolness=}')
                    yield(probe, coolness)

                    at_least_one_impact = True

        if phase % 2 == 0 and at_least_one_impact:
            phase += 1
        elif phase % 2 == 1 and not at_least_one_impact:
            phase += 1

        if phase == 4:
            break


@app.command()
def part1(file: typer.FileText):

    target = Rect.from_file(file)
    logging.info(target)

    most_cool = None

    for probe, coolness in all_impacts(target):
        if most_cool is None or coolness > most_cool:
            logging.info(f'part1: New coolest impact! {probe=}, {coolness=}')
            most_cool = coolness

    print(most_cool)


@app.command()
def part2(file: typer.FileText):

    target = Rect.from_file(file)
    logging.info(target)

    valid_probes = []

    for probe, coolness in all_impacts(target):
        valid_probes.append(probe)

    logging.info('All valid probes:')
    for probe in valid_probes:
        logging.info(probe)

    print(len(valid_probes))


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
