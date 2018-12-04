#!/usr/bin/env python3

import collections
from region import all_regions

counts = collections.defaultdict(lambda : 0)

for region in all_regions:
    for x in range(region.left, region.left + region.width):
        for y in range(region.top, region.top + region.height):
            counts[x, y] += 1

overlapping = 0

for x, y in counts:
    if counts[x, y] > 1:
        overlapping += 1

print(overlapping)
