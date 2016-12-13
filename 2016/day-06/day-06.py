#!/usr/bin/env python3

import argparse
from collections import defaultdict as ddict

parser = argparse.ArgumentParser()
parser.add_argument('input')
parser.add_argument('-m', '--mode', default = 'max', help = 'Use the most (max) or least (min) common letter')
args = parser.parse_args()

counters = ddict(lambda : ddict(lambda : 0))

with open(args.input, 'r') as fin:
    for line in fin:
        for index, character in enumerate(line.strip()):
            counters[index][character] += 1

if args.mode.lower().startswith('min'):
    comparator = min
elif args.mode.lower().startswith('max'):
    comparator = max
else:
    raise Exception('Unknown --mode {}, options are min and max'.format(args.mode))

print(''.join(
    comparator((count, character) for character, count in counters[index].items())[1]
    for index in counters
))
