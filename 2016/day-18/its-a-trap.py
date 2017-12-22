#!/usr/bin/env python3

import argparse
import logging

parser = argparse.ArgumentParser()
parser.add_argument('--initial', required = True, help = 'Initial state, a string of . (safe) and ^ (trap)')
parser.add_argument('--height', type = int, required = True, help = 'How tall of a room to generate')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

# If the previous row left, center, right looks like this, the next element is a trap
trap_map = {
    ('^', '^', '.'), # Its left and center tiles are traps, but its right tile is not.
    ('.', '^', '^'), # Its center and right tiles are traps, but its left tile is not.
    ('^', '.', '.'), # Only its left tile is a trap.
    ('.', '.', '^'), # Only its right tile is a trap.
}

def next(row):
    return ''.join(
        '^' if previous in trap_map else '.'
        for previous in zip('.' + row, row, row[1:] + '.')
    )

def rows(row, height):
    yield row
    for i in range(height - 1):
        row = next(row)
        yield row

safe_count = 0
trap_count = 0

for row in rows(args.initial, args.height):
    safe_count += row.count('.')
    trap_count += row.count('^')
    logging.info(row)

print('{} safe, {} trap'.format(safe_count, trap_count))
