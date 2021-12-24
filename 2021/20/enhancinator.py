import base64
import logging
import typer
import pathlib
import sys

from collections import *
from dataclasses import dataclass
from typing import *

app = typer.Typer()


def disableable_cache(x): return x


Point = Tuple[int, int]
BitIndex = List[bool]

NEIGHBORHOOD = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0), (0,  0), (1,  0),
    (-1,  1), (0,  1), (1,  1),
]


@dataclass(frozen=True)
class InfiniteBitmap:
    '''
    Store an infinitely large bitmap.
    
    data is 'known' values
    infinity is every other point off to infinity
    '''

    data: MutableMapping[Point, bool]
    infinity: bool

    @staticmethod
    def read(file: TextIO) -> 'InfiniteBitmap':
        '''Read an infinite bitmap from file. Assume unset bits (infinity) are False.'''

        logging.info('Reading infinity bitmap')

        data = {
            (x, y): c == '#'
            for y, line in enumerate(file)
            for x, c in enumerate(line.strip())
        }

        mid_x = max(x for x, _ in data) // 2
        mid_y = max(y for _, y in data) // 2

        return InfiniteBitmap({(x - mid_x, y - mid_y): bit for (x, y), bit in data.items()}, False)

    def __repr__(self):
        '''Return a much smaller representation.'''

        return f'InfiniteBinary<{self.infinity}, {len(self)}/{len(self.data)}, {self.bounds()}>'

    def __len__(self) -> int:
        '''Return the number of lit pixels (might be effectively infinite).'''

        if self.infinity:
            return sys.maxsize
        else:
            return sum(
                1 if self[p] else 0
                for p in self.data
            )

    def __getitem__(self, p: Point) -> bool:
        '''Get the value at a given point p (use infinity if the point isn't otherwise known).'''

        return self.data.get(p, self.infinity)

    def bounds(self) -> Tuple[int, int, int, int]:
        '''Return (minimum x, maximum x, minimum y, maximum y)'''

        return (
            min(x for x, _ in self.data) - 5,
            max(x for x, _ in self.data) + 5,
            min(y for _, y in self.data) - 5,
            max(y for _, y in self.data) + 5,
        )

    def render(self, include_axis: bool = False) -> 'Image':
        '''Render this infinity bitmap as an image.'''

        from PIL import Image  # type: ignore

        min_x, max_x, min_y, max_y = self.bounds()
        width = max_x - min_x + 1
        height = max_y - min_y + 1

        WHITE = (255, 255, 255)
        BLACK = (0, 0, 0)
        GRAY = (127, 127, 127)

        image = Image.new('RGB', (width, height), WHITE)
        pixels = image.load()

        for x in range(width):
            for y in range(height):
                ix = x + min_x
                iy = y + min_y

                if self[ix, iy]:
                    pixels[x, y] = BLACK
                elif include_axis and (ix == 0 or iy == 0):
                    pixels[x, y] = GRAY

        return image


@dataclass(frozen=True)
class BinaryMapping:
    '''Store a mapping from 9-bit values to 1-bit, loaded from a 512 character string of . (0) and # (1).'''

    map: List[bool]

    @staticmethod
    def read(file: TextIO):
        '''Read a BinaryMapping from an input stream.'''

        logging.info('Reading binary mapping')

        return BinaryMapping([c == '#' for c in file.readline().strip()])

    def __repr__(self):
        '''Return a unique representation of this map.'''

        binary = ''.join('1' if bit else '0' for bit in self.map)
        integer = int(binary, 2)
        bytes = integer.to_bytes(64, 'big')
        b64 = base64.b64encode(bytes).decode()

        return f'BinaryMapping<{b64}>'

    def __getitem__(self, k: Union[int, BitIndex]) -> bool:
        '''Get the value of the BinaryMapping by either integer index of a 9-bit binary BitIndex.'''

        if isinstance(k, int):
            return self.map[k]
        else:
            # TODO: Make this more efficient
            return self.map[int(''.join('1' if bit else '0' for bit in k), 2)]

    def __call__(self, bitmap: InfiniteBitmap) -> InfiniteBitmap:
        '''Apply this mapping to an infinite bitmap, generating a new one with this applied.'''

        logging.info(f'Calling {self} on {bitmap}')

        # Calculate the new infinity

        # If the lowest mapping is set, infinity goes from off to on
        # Likewise on the highest mapping for infinity from on to off
        if not bitmap.infinity and self[0]:
            new_infinity = True
        elif bitmap.infinity and not self[511]:
            new_infinity = False
        else:
            new_infinity = bitmap.infinity

        # Calculate all new points
        new_data = {}

        # Have to calculate all pixels one level out as well
        for x, y in bitmap.data:
            for xd, yd in NEIGHBORHOOD:
                # Don't calculate points more than once per update
                center = (x + xd, y + yd)
                if center in new_data:
                    continue

                neighbors = [
                    bitmap[x + xd + xd2, y + yd + yd2]
                    for xd2, yd2 in NEIGHBORHOOD
                ]

                new_value = self[neighbors]

                # If the value wasn't in the old map and matches the new infinity
                # We don't need to include it (prevent infinity expansion)
                if center in bitmap.data and new_value == new_infinity:
                    continue

                new_data[center] = new_value

        return InfiniteBitmap(new_data, new_infinity)


@app.command()
def render(file: typer.FileText, size: str, target: pathlib.Path, generations: int):

    from PIL import Image  # type: ignore

    f = BinaryMapping.read(file)
    file.readline()
    bitmap = InfiniteBitmap.read(file)

    try:
        width, height = [int(v) for v in size.split('x')]
    except:
        typer.echo('Unable to parse size, expecting a value like 400x400', err=True)
        raise typer.Exit(1)

    base_image = bitmap.render(True).resize((width, height), Image.NEAREST)

    rest_images = []
    for i in range(1, generations + 1):
        bitmap = f(bitmap)
        rest_images.append(bitmap.render(True).resize((width, height), Image.NEAREST))

    if str(target).lower().endswith('gif'):
        base_image.save(target, save_all=True, append_images=rest_images)
    else:
        for i, image in enumerate([base_image] + rest_images):
            image.save(str(target).format(i=i))


@app.command()
def find_emptiness(file: typer.FileText):

    f = BinaryMapping.read(file)
    file.readline()
    bitmap = InfiniteBitmap.read(file)

    generation = 0
    while len(bitmap):
        bitmap = f(bitmap)
        generation += 1

    print(generation)


@app.command()
def part1(file: typer.FileText):

    f = BinaryMapping.read(file)
    file.readline()
    bitmap = InfiniteBitmap.read(file)

    final_bitmap = f(f(bitmap))

    print(len(final_bitmap))


@app.command()
def part2(file: typer.FileText):
    f = BinaryMapping.read(file)
    file.readline()
    bitmap = InfiniteBitmap.read(file)

    for i in range(50):
        bitmap = f(bitmap)

    print(len(bitmap))


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
