#!/usr/bin/env python3

import sys; sys.path.insert(0, '..'); import lib

# https://www.redblobgames.com/grids/hexagons/
# Using a cube coordinate system

#   \ n  /
# nw +--+ ne
#   / y  \
# -+    x +-
#   \ z  /
# sw +--+ se
#   / s  \

origin = (0, 0, 0)

neighbors = {
    'n' : (0, 1, -1),
    'ne': (1, 0, -1),
    'se': (1, -1, 0),
    's' : (0, -1, 1),
    'sw': (-1, 0, 1),
    'nw': (-1, 1, 0),
}

def add(p1, p2):
    x1, y1, z1 = p1
    x2, y2, z2 = p2
    return (x1 + x2, y1 + y2, z1 + z2)

def move(p, d):
    return add(p, neighbors[d.strip()])

def distance(p1, p2):
    x1, y1, z1 = p1
    x2, y2, z2 = p2
    return max(abs(x1 - x2), abs(y1 - y2), abs(z1 - z2))

for line in lib.input():
    point = origin
    furthest_distance = 0
    furthest_point = None

    for direction in line.split(','):
        lib.log('Moving {} from {}', direction, point)
        point = move(point, direction)

        distance_to_origin = distance(origin, point)
        if distance_to_origin > furthest_distance:
            furthest_distance = distance_to_origin
            furthest_point = point

    lib.log('Ended at {}', point)
    print(f'ended at {point} ({distance_to_origin} from origin); furthest was {furthest_point} ({furthest_distance} from origin)')
