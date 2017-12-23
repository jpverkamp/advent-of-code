#!/usr/bin/env python3

import argparse
import fileinput
import logging

import sys; sys.path.insert(0, '..'); import lib

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*')
parser.add_argument('--function', default = '1', help = 'Function for offset, variables: size')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

for line in fileinput.input(args.files):
    line = line.strip()

    total = 0

    offset = lib.math(args.function, {'size': len(line)})

    for c1, c2 in zip(line, line[offset:] + line[:offset]):
        if c1 == c2:
            total += int(c1)

    print(total)
