#!/usr/bin/env python3

import itertools

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('index')

if lib.part(1):
    grid = lib.SpiralGrid()

    x, y = grid[lib.param('index')]
    print(abs(x) + abs(y))

if lib.part(2):
    grid = lib.SpiralGrid()

    values = {}
    values[0, 0] = 1

    for i in itertools.count(2):
        x, y = grid[i]

        # Calculate sum of neighbor values we've already calculated for the new value
        sum_of_neighbors = sum(
            values[x + xd, y + yd]
            for xd in (-1, 0, 1)
            for yd in (-1, 0, 1)
            if (not xd == yd == 0) and (x + xd, y + yd) in values
        )
        values[x, y] = sum_of_neighbors

        lib.log('Point {} {} has value {}', i, (x, y), sum_of_neighbors)

        # As soon as we see one bigger than the given index, bail out
        if sum_of_neighbors > int(lib.param('index')):
            print(sum_of_neighbors)
            break
