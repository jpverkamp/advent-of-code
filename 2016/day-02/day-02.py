#!/usr/bin/env python3

import argparse
import sys

parser = argparse.ArgumentParser()
parser.add_argument('grid_file')
parser.add_argument('input_file')
args = parser.parse_args()

grid = {}
location = None
deltas = {'U':  0-1j, 'L': -1+0j, 'R':  1+0j, 'D':  0+1j}
code = ''

with open(args.grid_file, 'r') as fin:
    for imag, line in enumerate(fin):
        if not line.strip(): continue

        for real, char in enumerate(line.strip()):
            if char != '-':
                grid[complex(real, imag)] = char

            if char == '5':
                location = complex(real, imag)

with open(args.input_file, 'r') as fin:
    for line in fin:
        for command in line.strip():
            new_location = location + deltas[command]
            if new_location in grid:
                location = new_location

        code += str(grid[location])

print(code)
