#!/usr/bin/env python3

import queue

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--key', required = True)

# Generate a grid of bits based on knothashes of the input
data = []
for row in range(128):
    value = '{}-{}'.format(lib.param('key'), row)
    hash = lib.knothash(value)
    bits = lib.hex2bits(hash)

    lib.log(f'{value} {hash} {bits}')

    data.append(bits)

# Calculate how many 1 bits we have for part 1
used_count = sum(
    1 if bit == '1' else 0
    for row in data
    for bit in row
)
print(f'{used_count} used')

# Make a map of point to regions
def get_region(point):
    '''Flood fill a region from a given point, yield all points in the same region.'''

    nodes = set()
    q = queue.Queue()
    q.put(point)

    while not q.empty():
        point = q.get()

        if point in nodes:
            continue
        else:
            yield point
            nodes.add(point)

        x, y = point
        for xd, yd in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            if 0 <= x + xd < 128 and 0 <= y + yd < 128 and data[x + xd][y + yd] == '1':
                q.put((x + xd, y + yd))

point_to_region = {}
region_to_point = {}

for x in range(128):
    for y in range(128):
        point = (x, y)
        region_label = 0

        # Expand points that haven't already been labeled
        if data[x][y] == '1' and point not in point_to_region:
            points = set(get_region(point))
            region = len(region_to_point)

            #lib.log(f'New region ({region}) seeded by {point} contains {len(points)} points: {points}')

            region_to_point[region] = points

            for point in points:
                point_to_region[point] = region

lib.log('\n'.join(
    ''.join(
        chr(19968 + point_to_region[x, y]) if (x, y) in point_to_region else chr(12288)
        for y in range(128)
    ) for x in range(128)
))

region_count = len(region_to_point)
print(f'{region_count} regions')
