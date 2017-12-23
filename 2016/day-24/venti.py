#!/usr/bin/env python3

import argparse
import itertools
import fileinput
import logging
import queue
import tabulate

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*')
parser.add_argument('--must-return', action = 'store_true', help = 'The robot has to return to point 0')
parser.add_argument('--debug', action = 'store_true')
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

walls = set()
name_to_point = {}
point_to_name = {}

# Load the input file into a set of walls and the location of points of interest
logging.info('Loading map...')

for y, line in enumerate(fileinput.input(args.files)):
    for x, c in enumerate(line.strip()):
        if c.isdigit():
            name_to_point[int(c)] = (x, y)
            point_to_name[(x, y)] = int(c)

        elif c == '#':
            walls.add((x, y))

# Dynamically fill a distance map to a given point
logging.info('Calculating distance map...')

def distances_to(name):
    to_scan = queue.Queue()
    to_scan.put((name_to_point[name], 0))

    scanned = set()

    result = {}

    while not to_scan.empty():
        point, distance = to_scan.get()

        if point in point_to_name:
            name = point_to_name[point]
            if name not in result:
                result[name] = distance

        if point in scanned:
            continue
        else:
            scanned.add(point)

        x, y = point
        for xd, yd in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            neighbor = (x + xd, y + yd)
            if neighbor not in walls:
                to_scan.put((neighbor, distance + 1))

    return result

distances = {
    name: distances_to(name)
    for name in name_to_point
}
names = list(sorted(name_to_point.keys()))

tabulate_date = [
    [distances[x][y] for x in names]
    for y in names
]

logging.info('Distance map:\n{}'.format(tabulate.tabulate(tabulate_date, headers = 'keys', showindex = 'always', tablefmt = 'fancy_grid')))

# Try all orderings of points
logging.info('Trying all orderings...')

def total_length(ordering):
    # If we have to return back to the origin, the distance will from the last point to 0
    offset_ordering = ordering[1:]
    if args.must_return:
        offset_ordering += ordering[:1]

    return sum(
        distances[p1][p2]
        for p1, p2 in zip(ordering, offset_ordering)
    )

minimum_length = float("inf")
minimum_ordering = None

# Looks a bit funny since we have to start at 0
for ordering in itertools.permutations(names[1:], len(names) - 1):
    ordering = [0] + list(ordering)
    length = total_length(ordering)
    if not minimum_ordering or length < minimum_length:
        logging.info('New best ({} steps): {}'.format(length, ordering))
        minimum_length = length
        minimum_ordering = ordering

print('Best ordering ({} steps): {}'.format(minimum_length, minimum_ordering))
