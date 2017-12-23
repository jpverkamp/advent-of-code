#!/usr/bin/env python3

import re

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--no-anagrams', action = 'store_true')

valid_count = 0
total_count = 0

for line in lib.input():
    total_count += 1

    words = line.split()
    if lib.param('no_anagrams'):
        words = [''.join(sorted(word)) for word in words]

    if len(words) == len(set(words)):
        lib.log('{} is valid', line)
        valid_count += 1
    else:
        lib.log('{} is invalid', line)

print(valid_count)
