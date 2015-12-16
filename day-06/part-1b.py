#!/usr/bin/env python3

import re
import sys

filters = []
for line in sys.stdin:
	m = re.match('(.*) (\d+),(\d+) through (\d+),(\d+)', line.strip())
	filters.append([m.group(1)] + list(map(int, m.groups()[1:])))

def is_on(x, y):
	on = False
	for mode, x1, y1, x2, y2 in filters:
		if x1 <= x <= x2 and y1 <= y <= y2:
			if mode == 'turn on':
				on = True
			elif mode == 'turn off':
				on = False
			else:
				on = not on
	return on

print(sum(
	1 if is_on(x, y) else 0
	for x in range(1000)
	for y in range(1000)
))
