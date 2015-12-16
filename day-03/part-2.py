#!/usr/bin/env python3

import collections
import sys

presents = collections.defaultdict(lambda : 0)
presents[0+0j] = 2

locations = [
	0+0j,
	0+0j,
]

directions = {
	'<': -1+0j,
	'>':  1+0j,
	'^':  0+1j,
	'v':  0-1j,
}

for i, c in enumerate(sys.stdin.read()):
	which = i % len(location)
	locations[which] += directions.get(c, 0+0j)
	presents[locations[which]] += 1

print(len(presents))
