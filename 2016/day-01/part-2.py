#!/usr/bin/env python3

import argparse
import sys

parser = argparse.ArgumentParser()
parser.add_argument('input_file')
args = parser.parse_args()

location = 0+0j
facing = 1+0j
visited = {0+0j}

rotations = {'R': 0+1j, 'L': 0-1j}

with open(args.input_file, 'r') as fin:
    for command in fin.read().split(', '):
        facing *= rotations[command[0]]

        for step in range(int(command[1:])):
            location += facing

            if location in visited:
                print(abs(int(location.real)) + abs(int(location.imag)))
                sys.exit(0)

            visited.add(location)
