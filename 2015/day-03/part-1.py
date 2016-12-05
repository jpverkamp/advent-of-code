#!/usr/bin/env python3

import collections
import sys

presents = collections.defaultdict(lambda : 0)
location = 0+0j

directions = {
	'<': -1+0j,
	'>':  1+0j,
	'^':  0+1j,
	'v':  0-1j,
}

for c in sys.stdin.read():
	location += directions.get(c, 0+0j)
	presents[location] += 1

print(len(presents))
