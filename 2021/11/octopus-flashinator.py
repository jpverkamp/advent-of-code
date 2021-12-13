from pathlib import Path
import typer

from dataclasses import dataclass
from typing import *

app = typer.Typer()

NEIGHBORS = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 0), (0, 1),
    (1, -1), (1, 0), (1, 1),
]


@dataclass
class Cavern:
    '''Simulation for https://adventofcode.com/2021/day/11.'''

    width: int
    height: int
    data: MutableMapping[Tuple[int, int], int]

    @staticmethod
    def from_file(file: TextIO):
        '''Load a simulation from a file-like object.'''

        data = {
            (x, y): int(value)
            for x, line in enumerate(file)
            for y, value in enumerate(line.strip())
        }
        width = max(x + 1 for (x, _) in data)
        height = max(y + 1 for (_, y) in data)

        return Cavern(width, height, data)

    def step(self):
        '''Advance the simulation 1 step, return the number of flashes.'''

        flashpoint = 0

        # First advance everyone 1
        for (x, y) in self.data:
            self.data[x, y] += 1

        # Repeatedly find any 9s, but only trigger each one once (advanced)
        advanced = set()
        while True:

            # Find the set of points that haven't been advanced and should
            to_advance = {
                (x, y)
                for (x, y) in self.data
                if (x, y) not in advanced and self.data[x, y] > 9
            }

            # If we didn't, we're done
            if not to_advance:
                break

            # If we did, increment each neighbor
            for (x, y) in to_advance:
                flashpoint += 1

                for (xd, yd) in NEIGHBORS:
                    if (x + xd, y + yd) not in self.data:
                        continue

                    self.data[x + xd, y + yd] += 1

                advanced.add((x, y))

        # Once we're out of the loop, set all points that actually advanced (hit 9) to 0
        for (x, y) in advanced:
            self.data[x, y] = 0

        return flashpoint

    def __str__(self):
        return '\n'.join(
            ''.join(str(self.data[x, y]) for y in range(self.height))
            for x in range(self.width)
        ) + '\n'


@app.command()
def part1(file: typer.FileText):

    cavern = Cavern.from_file(file)
    flashpoint = 0

    for i in range(100):
        flashpoint += cavern.step()

    print(flashpoint)


@app.command()
def part2(file: typer.FileText):

    cavern = Cavern.from_file(file)
    generation = 0

    while True:
        generation += 1
        flashpoint = cavern.step()

        if flashpoint == cavern.width * cavern.height:
            break

    print(generation)


@app.command()
def animate(file: typer.FileText, generations: int, filename: Path):
    from PIL import Image  # type: ignore

    SCALE = 8

    cavern = Cavern.from_file(file)
    frames = []

    def add_frame():
        image = Image.new('RGB', (cavern.width, cavern.height), (0, 0, 0))
        pixels = image.load()

        for x in range(cavern.width):
            for y in range(cavern.height):
                value = int(255 * cavern.data[x, y] / 10)
                pixels[x, y] = (value, value, value)

        frames.append(image.resize((cavern.width * SCALE, cavern.height * SCALE), Image.NEAREST))

    add_frame()

    for _ in range(generations):
        cavern.step()
        add_frame()

    frames[0].save(filename, save_all=True, loop=0, duration=40, append_images=frames[1:])


if __name__ == '__main__':
    app()
