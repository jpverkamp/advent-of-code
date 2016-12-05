#!/usr/bin/env python3

import argparse
import sys

parser = argparse.ArgumentParser()
parser.add_argument('input_file')
args = parser.parse_args()

possible_triangles = 0

def rotate(stream):
    while True:
        triple = []
        for i in range(3):
            row = stream.readline()
            if not row: return
            triple.append(list(map(int, row.split())))

        for row in range(3):
            yield list(sorted(triple[col][row] for col in range(3)))

with open(args.input_file, 'r') as fin:
    for sides in rotate(fin):
        if sides[0] + sides[1] > sides[2]:
            possible_triangles += 1

print(possible_triangles)
