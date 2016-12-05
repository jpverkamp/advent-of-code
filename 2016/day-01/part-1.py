#!/usr/bin/env python3

import argparse

parser = argparse.ArgumentParser()
parser.add_argument('input_file')
args = parser.parse_args()

location = 0+0j
facing = 1+0j

rotations = {'R': 0+1j, 'L': 0-1j}

with open(args.input_file, 'r') as fin:
    for command in fin.read().split(', '):
        facing *= rotations[command[0]]
        location += facing * int(command[1:])

print(abs(int(location.real)) + abs(int(location.imag)))
