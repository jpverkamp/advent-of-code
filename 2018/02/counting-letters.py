#!/usr/bin/env python3

import collections
import fileinput

def count_letters(word):
    counts = collections.defaultdict(lambda : 0)
    for letter in word.strip():
        counts[letter] += 1
    return counts

count_2s = 0
count_3s = 0

for word in fileinput.input():
    counts = count_letters(word)
    if 2 in counts.values(): count_2s += 1
    if 3 in counts.values(): count_3s += 1

print(count_2s * count_3s)
