#!/usr/bin/env python3

import argparse

parser = argparse.ArgumentParser()
parser.add_argument('input_file')
args = parser.parse_args()

possible_triangles = 0

with open(args.input_file, 'r') as fin:
    for line in fin:
        sides = list(sorted(map(int, line.split())))
        if sides[0] + sides[1] > sides[2]:
            possible_triangles += 1

print(possible_triangles)
