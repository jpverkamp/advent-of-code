#!/usr/bin/env python3

import argparse
from collections import defaultdict as ddict

parser = argparse.ArgumentParser()
parser.add_argument('input')
args = parser.parse_args()

counters = ddict(lambda : ddict(lambda : 0))

with open(args.input, 'r') as fin:
    for line in fin:
        for index, character in enumerate(line.strip()):
            counters[index][character] += 1

for comparator in (max, min):
    print(comparator.__name__, ''.join(
        comparator((count, character) for character, count in counters[index].items())[1]
        for index in counters
    ))
