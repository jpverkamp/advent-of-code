#!/usr/bin/env python3

import collections
import itertools
import re
import sys

happiness = collections.defaultdict(lambda : collections.defaultdict(lambda : 0))
for line in sys.stdin:
    self, gain_lose, amount, other = re.match(
        r'(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).',
        line
    ).groups()

    amount = int(amount)
    if gain_lose == 'lose':
        amount *= -1

    happiness[self][other] = amount

best_score, best_ordering = max(
    (
        sum(
            happiness[a][b] + happiness[b][a]
            for a, b in zip(ordering, ordering[1:] + (ordering[0],))
        ),
        ordering
    )
    for ordering in itertools.permutations(happiness.keys())
)

print(best_ordering)
print(best_score)
