#!/usr/bin/env python3

import re
import sys

lights = [
	[False for y in range(1000)]
	for x in range(1000)
]

for line in sys.stdin:
	mode, x1, y1, x2, y2 = re.match('(.*) (\d+),(\d+) through (\d+),(\d+)', line.strip()).groups()
	x1, y1, x2, y2 = map(int, (x1, y1, x2, y2))

	for x in range(x1, x2 + 1):
		for y in range(y1, y2 + 1):
			if mode == 'turn on':
				lights[x][y] = True
			elif mode == 'turn off':
				lights[x][y] = False
			else:
				lights[x][y] = not lights[x][y]

print(sum(
	1 if lights[x][y] else 0
	for x in range(1000)
	for y in range(1000)
))
