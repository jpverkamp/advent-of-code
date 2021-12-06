import collections
import copy
import math
import itertools
import typer

from dataclasses import dataclass
from typing import List, TextIO, MutableMapping

app = typer.Typer()


@dataclass(frozen=True)
class Point:
    x: int
    y: int


@dataclass(frozen=True)
class Line:
    p1: Point
    p2: Point

    def is_vertical(self):
        return self.p1.x == self.p2.x

    def is_horizontal(self):
        return self.p1.y == self.p2.y

    def is_orthagonal(self):
        return self.is_vertical() or self.is_horizontal()

    def points(self):
        # TODO: handle lines that aren't vertical, horizontal, or diagonal

        xd = 0 if self.p1.x == self.p2.x else (1 if self.p1.x < self.p2.x else -1)
        yd = 0 if self.p1.y == self.p2.y else (1 if self.p1.y < self.p2.y else -1)

        p = self.p1
        while p != self.p2:
            yield p
            p = Point(p.x + xd, p.y + yd)

        yield p


def parse(file: TextIO) -> List[Line]:
    result = []

    for line in file:
        x1, y1, x2, y2 = [int(v) for v in line.replace(' -> ', ',').split(',')]
        result.append(Line(Point(x1, y1), Point(x2, y2)))

    return result


@app.command()
def part1(file: typer.FileText):

    lines = parse(file)
    counter: MutableMapping[Point, int] = collections.Counter()

    for line in lines:
        if not line.is_orthagonal():
            continue

        for point in line.points():
            counter[point] += 1

    print(sum(1 if count > 1 else 0 for point, count in counter.items()))


@app.command()
def part2(file: typer.FileText):

    lines = parse(file)
    counter: MutableMapping[Point, int] = collections.Counter()

    for line in lines:
        for point in line.points():
            counter[point] += 1

    print(sum(1 if count > 1 else 0 for point, count in counter.items()))


if __name__ == '__main__':
    app()
