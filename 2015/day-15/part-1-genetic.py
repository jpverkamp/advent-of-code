#!/usr/bin/env python3

# NOTE: Doesn't actually return the correct solution even when given the same
#       amount of time as the brute force solution, but it's still neat.

import collections
import random
import sys

try:
    TOTAL_QUANTITY = int(sys.argv[1])
    ITERATIONS_BEFORE_SHUFFLE = 1000
    SHUFFLES_BEFORE_EXIT = 100
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

split = [TOTAL_QUANTITY // len(items)] * (len(items) - 1) + [TOTAL_QUANTITY - (len(items) - 1) * TOTAL_QUANTITY // len(items)]

best_quantities = dict(zip(items, split))
best_score = calculate_score(best_quantities)

print('initial state:', best_score, best_quantities)

since_last_update = 0
times_scrambled = 0

while True:
    since_last_update += 1

    # Choose two endpoints to change some values from
    # i will give up items, thus must have some to give up
    i = j = random.randrange(0, len(items))
    while split[i] == 1:
        i = j = random.randrange(0, len(items))

    # j will take the items, cannot be the same as i
    while i == j:
        j = random.randrange(0, len(items))

    try:
        amt = random.randrange(1, split[i])
    except:
        i, j = j, i
        amt = random.randrange(1, split[i])

    split[i] -= amt
    split[j] += amt

    current_quantities = dict(zip(items, split))
    current_score = calculate_score(current_quantities)

    if current_score > best_score:
        best_score = current_score
        best_quantities = current_quantities
        since_last_update = 0

    if since_last_update > ITERATIONS_BEFORE_SHUFFLE:
        if times_scrambled < SHUFFLES_BEFORE_EXIT:
            random.shuffle(split)
            since_last_update = 0
            times_scrambled += 1

        else:
            break

print(best_score)
print(best_quantities)
