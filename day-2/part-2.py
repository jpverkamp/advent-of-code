#!/usr/bin/env python3

import sys

total_ribbon = 0

for line in sys.stdin:
	l, w, h = list(sorted(map(int, line.strip().split('x'))))

	total_ribbon += max(
		2 * (l + w), # smallest distance around sides
		4 * l, # smallest perimeter
	)

	total_ribbon += l * w * h

print(total_ribbon)
