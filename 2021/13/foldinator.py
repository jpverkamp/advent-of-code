import typer

from collections import *
from dataclasses import dataclass
from functools import cache
from typing import *

app = typer.Typer()


@dataclass(frozen=True)
class Point:
    x: int
    y: int

    def __repr__(self):
        return f'<{self.x}, {self.y}>'


@dataclass(frozen=True)
class Fold:
    horizontal: bool
    line: int

    def apply_to(self, points: Set[Point]) -> Set[Point]:
        return {
            Point(
                p.x if (self.horizontal or p.x < self.line) else self.line * 2 - p.x,
                p.y if (not self.horizontal or p.y < self.line) else self.line * 2 - p.y
            )
            for p in points
        }

    def __repr__(self):
        return f'fold@{"x" if self.horizontal else "y"}={self.line}'


def load(file: TextIO) -> Tuple[Set[Point], List[Fold]]:
    points = set()
    folds = []

    for line in file:
        if ',' in line:
            xs, ys = line.split(',')
            points.add(Point(x=int(xs), y=int(ys)))

        elif '=' in line:
            left, vs = line.split('=')
            folds.append(Fold(horizontal=left.endswith('y'), line=int(vs)))

    return points, folds


def render(points: Set[Point]):
    width = max(p.x + 1 for p in points)
    height = max(p.y + 1 for p in points)

    print('\n'.join(
        ''.join(
            '*' if Point(x, y) in points else ' '
            for x in range(width)
        )
        for y in range(height)
    ))
    print()


def apply_fold(points: Set[Point], fold: Fold) -> Set[Point]:
    pass


@app.command()
def part1(file: typer.FileText):
    points, folds = load(file)
    points = folds[0].apply_to(points)
    print(len(points))


@app.command()
def part2(file: typer.FileText):
    points, folds = load(file)

    for fold in folds:
        points = fold.apply_to(points)

    render(points)


if __name__ == '__main__':
    app()
