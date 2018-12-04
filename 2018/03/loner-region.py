#!/usr/bin/env python3

from region import Region, all_regions

def overlaps(r1, r2):
    return not (
        r1.left + r1.width <= r2.left
        or r2.left + r2.width <= r1.left
        or r1.top + r1.height <= r2.top
        or r2.top + r2.height <= r1.top
    )
Region.overlaps = overlaps

for r1 in all_regions:
    found_overlap = False

    for r2 in all_regions:
        if r1 == r2:
            continue

        if r1.overlaps(r2):
            found_overlap = True
            continue

    if not found_overlap:
        print(r1)
        exit(0)
