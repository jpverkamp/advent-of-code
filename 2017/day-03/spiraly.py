#!/usr/bin/env python3

import argparse
import itertools
import fileinput
import logging

import sys; sys.path.insert(0, '..'); import lib

parser = argparse.ArgumentParser()
parser.add_argument('index')
parser.add_argument('--part', default = (1, 2), type = int, nargs = '+', choices = (1, 2))
parser.add_argument('--debug', action = 'store_true')
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

if 1 in args.part:
    grid = lib.SpiralGrid()

    x, y = grid[args.index]
    print(abs(x) + abs(y))

if 2 in args.part:
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

        logging.info('Point {} {} has value {}'.format(i, (x, y), sum_of_neighbors))

        # As soon as we see one bigger than the given index, bail out
        if sum_of_neighbors > int(args.index):
            print(sum_of_neighbors)
            break
