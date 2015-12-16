#!/usr/bin/env python3

import sys

total_area = 0

for line in sys.stdin:
    l, w, h = list(sorted(map(int, line.strip().split('x'))))
    area = 3 * l * w + 2 * w * h + 2 * h * l
    total_area += area

print(total_area)
