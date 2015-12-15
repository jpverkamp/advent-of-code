#!/usr/bin/env python3

import collections
import sys

try:
    TOTAL_QUANTITY = int(sys.argv[1])
except:
    print('Usage: cat input.txt | ./part-1.py [TOTAL_QUANTITY]')
    sys.exit(0)

ingredients = collections.defaultdict(lambda : collections.defaultdict(lambda : 0))
properties = set()

for line in sys.stdin:
    item, pairs = line.split(':')
    for pair in pairs.split(','):
        property, amount = pair.strip().split(' ')
        ingredients[item.lower()][property] = int(amount)
        properties.add(property)

items = list(sorted(ingredients.keys()))
properties.remove('calories')

def calculate_score(quantities):
    score = 1

    for property in properties:
        property_score = sum(
            quantities[item] * ingredients[item][property]
            for item in quantities
        )

        if property_score > 0:
            score *= property_score

    return score

def splits(amount, count):
    if count <= 1:
        yield [amount]
    else:
        for i in range(amount + 1):
            for subsplit in splits(amount - i, count - 1):
                yield [i] + subsplit

best_score = 0
best_quantities = None

for split in splits(TOTAL_QUANTITY, len(items)):
    quantities = dict(zip(items, split))
    score = calculate_score(quantities)

    if score > best_score:
        best_score = score
        best_quantities = quantities

print(best_score)
print(best_quantities)
