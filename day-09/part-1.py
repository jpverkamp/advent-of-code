#!/usr/bin/env python3

import collections
import itertools
import re
import sys

routes = collections.defaultdict(lambda : collections.defaultdict(lambda : float("inf")))

for line in sys.stdin:
    src, dst, dist = re.match(r'(\w+) to (\w+) = (\d+)', line).groups()
    dist = int(dist)

    routes[src][dst] = dist
    routes[dst][src] = dist

best_length, best_ordering = min(
    (sum(routes[src][dst] for src, dst in zip(ordering, ordering[1:])), ordering)
    for ordering in itertools.permutations(routes.keys())
)

print(best_length)
