import logging
import typer
import itertools
import pathlib

from collections import *
from dataclasses import dataclass
from typing import *

app = typer.Typer()


def disableable_cache(x): return x


@dataclass(frozen=True)
class Point:
    x: int
    y: int

    def __repr__(self):
        return f'<{self.x}, {self.y}>'

    @disableable_cache
    def __add__(self, other: 'Point') -> 'Point':
        return Point(self.x + other.x, self.y + other.y)

    @disableable_cache
    def __mod__(self, bound: 'Point') -> 'Point':
        return Point(self.x % bound.x, self.y % bound.y)


EAST = Point(1, 0)
SOUTH = Point(0, 1)


@dataclass(frozen=True)
class State:
    bounds: Point
    east_movers: Set[Point]
    south_movers: Set[Point]

    @staticmethod
    def read(file: TextIO) -> 'State':
        east_movers = set()
        south_movers = set()

        for y, line in enumerate(file):
            for x, c in enumerate(line.strip()):
                if c == '>':
                    east_movers.add(Point(x, y))
                elif c == 'v':
                    south_movers.add(Point(x, y))

        return State(Point(x + 1, y + 1), east_movers, south_movers)

    def __str__(self):
        data = '\n'.join(
            ''.join(
                (
                    '>' if Point(x, y) in self.east_movers
                    else 'v' if Point(x, y) in self.south_movers
                    else '.'
                )
                for x in range(self.bounds.x)
            )
            for y in range(self.bounds.y)
        )

        return data

    def step(self) -> 'State':
        new_east_movers = {
            (p + EAST) % self.bounds if (
                (p + EAST) % self.bounds not in self.east_movers
                and (p + EAST) % self.bounds not in self.south_movers
            ) else p
            for p in self.east_movers
        }

        new_south_movers = {
            (p + SOUTH) % self.bounds if(
                (p + SOUTH) % self.bounds not in new_east_movers
                and (p + SOUTH) % self.bounds not in self.south_movers
            ) else p
            for p in self.south_movers

        }

        return State(self.bounds, new_east_movers, new_south_movers)

    def to_image(self):
        from PIL import Image  # type: ignore

        east_color = (255, 0, 0)
        south_color = (0, 255, 0)

        image = Image.new('RGB', (self.bounds.x, self.bounds.y), (0, 0, 0))
        pixels = image.load()

        for p in self.east_movers:
            pixels[p.x, p.y] = east_color

        for p in self.south_movers:
            pixels[p.x, p.y] = south_color

        return image


@app.command()
def solve(file: typer.FileText):
    s = State.read(file)

    for i in itertools.count(1):
        logging.info(f'{i}\n{s}')

        sp = s.step()
        if s == sp:
            break
        else:
            s = sp

    logging.info('\n{s}\n')
    print(f'{i} steps')


@app.command()
def render(file: typer.FileText, target: pathlib.Path, size: Optional[str] = typer.Argument(None)):
    from PIL import Image  # type: ignore

    s = State.read(file)

    try:
        if size is not None:
            width, height = [int(v) for v in size.split('x')]
        else:
            width, height = s.bounds.x, s.bounds.y
    except:
        typer.echo('Unable to parse size, expecting a value like 400x400', err=True)
        raise typer.Exit(1)

    base_image = s.to_image().resize((width, height), Image.NEAREST)

    rest_images = []
    for i in itertools.count(1):
        logging.info(f'{i}\n{s}')
        rest_images.append(s.to_image().resize((width, height), Image.NEAREST))

        sp = s.step()
        if s == sp:
            break
        else:
            s = sp

    if str(target).lower().endswith('gif'):
        base_image.save(target, save_all=True, loop=0, append_images=rest_images)
    else:
        for i, image in enumerate([base_image] + rest_images):
            image.save(str(target).format(i=i))


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
