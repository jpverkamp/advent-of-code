from itertools import product
import typer

from typing import MutableMapping, Tuple, TextIO, MutableSet
from pathlib import Path

app = typer.Typer()

ORTHAGONAL_NEIGHBORS = [(-1, 0), (1, 0), (0, -1), (0, 1)]


def load(file: TextIO) -> MutableMapping[Tuple[int, int], int]:
    return {
        (x, y): int(height)
        for y, line in enumerate(file)
        for x, height in enumerate(line.strip())
    }


def find_basins(heightmap):
    # A map (like heightmap) of Point -> which basin that point belongs to
    basinmap: MutableMapping[Tuple[int, int], int] = {}

    # A map of basin index to a set of points in that basin (to count size)
    basins: MutableMapping[int, MutableSet[Tuple[int, int]]] = {}

    def floodfill(x, y, value):
        to_visit = [(x, y)]
        basins[value] = set()

        while to_visit:
            x, y = to_visit.pop()

            # Ignore points out of bounds or with heights of 9
            if (x, y) not in heightmap or heightmap[x, y] == 9:
                continue

            # Don't fill a point twice
            if basinmap.get((x, y)):
                continue

            # Otherwise, fill it and recur
            #
            # This is a bit inefficient because we're adding the point we came from
            #   but because we only expand if we actually fill a point, it won't get stuck
            basinmap[x, y] = value
            basins[value].add((x, y))

            for xd, yd in ORTHAGONAL_NEIGHBORS:
                to_visit.append((x + xd, y + yd))

    # Flood fill from every non-9 point in the map
    next_value = 1
    for (x, y), height in heightmap.items():
        # Ignore 9s and anything that already has a value
        if height == 9 or (x, y) in basinmap:
            continue

        # If we made it this far, floodfill the next basin and increment
        floodfill(x, y, next_value)
        next_value += 1

    return basins


@app.command()
def part1(file: typer.FileText):

    heightmap = load(file)
    total_risk = 0

    for (x, y), height in heightmap.items():
        neighbor_heights = [
            heightmap.get((x + xd, y + yd), 10)
            for xd, yd in ORTHAGONAL_NEIGHBORS
        ]

        if min(neighbor_heights) > height:
            total_risk += height + 1

    print(f'{total_risk=}')


@app.command()
def part2(file: typer.FileText):

    heightmap = load(file)
    basins = find_basins(heightmap)

    # Find the size of the largest three basins
    sizes = list(reversed(sorted(len(points) for _, points in basins.items())))
    product = sizes[0] * sizes[1] * sizes[2]

    print(f'The largest basins are {sizes[:3]} with a size product of {product}')


@app.command()
def view(file: typer.FileText, heightmap_file: Path, basin_file: Path, largest_basin_file: Path):
    from PIL import Image  # type: ignore

    heightmap = load(file)
    image_width = max(x for (x, _) in heightmap) + 1
    image_height = max(y for (_, y) in heightmap) + 1

    # Generate a grayscale map of heights from the given image
    heightmap_image = Image.new('HSV', (image_width, image_height))
    heightmap_data = heightmap_image.load()

    for (x, y), height in heightmap.items():
        heightmap_data[x, y] = (0, 0, 255 * height // 10)

    heightmap_image = heightmap_image.resize((image_width * 4, image_height * 4), Image.NEAREST)
    heightmap_image.convert(mode='RGB').save(heightmap_file)

    # Generate a map of all of the basins in the image
    basinmap_image = Image.new('HSV', (image_width, image_height))
    basinmap_data = basinmap_image.load()

    basins = find_basins(heightmap)

    for index, points in basins.items():
        for (x, y) in points:
            basinmap_data[x, y] = ((10007 * index // len(basins)) % 255, 255, 255)

    basinmap_image = basinmap_image.resize((image_width * 4, image_height * 4), Image.NEAREST)
    basinmap_image.convert(mode='RGB').save(basin_file)

    # Only draw the 3 largest basins
    biggest_basins = [
        index
        for _, index in list(reversed(sorted(
            (len(points), index)
            for index, points in basins.items()
        )))[:3]
    ]

    biggest_basin_image = Image.new('HSV', (image_width, image_height))
    biggest_basin_data = biggest_basin_image.load()

    basins = find_basins(heightmap)

    for index, points in basins.items():
        for (x, y) in points:
            if index in biggest_basins:
                biggest_basin_data[x, y] = ((10007 * index // len(basins)) % 255, 255, 255)
            else:
                biggest_basin_data[x, y] = (0, 0, 255)

    biggest_basin_image = biggest_basin_image.resize((image_width * 4, image_height * 4), Image.NEAREST)
    biggest_basin_image.convert(mode='RGB').save(largest_basin_file)




if __name__ == '__main__':
    app()
