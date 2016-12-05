#!/usr/bin/env python3

import collections
import operator
import sys

targets = {}
sues = collections.defaultdict(dict)

comparators = collections.defaultdict(lambda : operator.eq)
comparators['cats'] = comparators['trees'] = operator.gt
comparators['pomeranians'] = comparators['goldfish'] = operator.lt

loading_targets = True
for line in sys.stdin:
    line = line.strip()

    if not line:
        loading_targets = False

    elif loading_targets:
        key, val = line.split(': ')
        targets[key] = int(val)

    else:
        sue, things = line.strip().split(': ', 1)

        for thing in things.split(', '):
            key, val = thing.split(': ')
            sues[sue][key] = int(val)

for sue in sues:
    valid = True

    for key in targets:
        if key in sues[sue] and not comparators[key](sues[sue][key], targets[key]):
                valid = False
                break

    if valid:
        print(sue)
