#!/usr/bin/env python3

import argparse
import fileinput
import logging

import sys; sys.path.insert(0, '..'); import lib

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*')
parser.add_argument('--part', default = (1, 2), type = int, nargs = '+', choices = (1, 2))
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

data = [
    [int(el) for el in row.split()]
    for row in fileinput.input(args.files)
]

# Part 1: Sum of differences between largest and smallest in each row
if 1 in args.part:
    print(sum(
        max(*row) - min(*row)
        for row in data
    ))

# Part 2: Result of dividing the only two evenly divisble numbers in a row
if 2 in args.part:
    print(sum(
        a // b
        for row in data
        for a in row
        for b in row
        if a != b and a % b == 0
    ))
