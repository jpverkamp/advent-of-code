#!/usr/bin/env python3

import argparse

parser = argparse.ArgumentParser()
parser.add_argument('input_file')
args = parser.parse_args()

location = 0+0j
facing = 1+0j

rotations = {'R': 0+1j, 'L': 0-1j}

visited = {0+0j}
first_duplicate = None

with open(args.input_file, 'r') as fin:
    for command in fin.read().split(', '):
        facing *= rotations[command[0]]

        for step in range(int(command[1:])):
            location += facing

            if location in visited:
                if first_duplicate == None:
                    first_duplicate = location
            else:
                visited.add(location)

def format_location(pt):
    return '({}, {}), {} blocks from the origin'.format(
        abs(int(pt.real)),
        abs(int(pt.imag)),
        abs(int(pt.real)) + abs(int(pt.imag)),
    )

print('final location:', format_location(location))
print('first duplicate:', format_location(first_duplicate))
