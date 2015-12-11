#!/usr/bin/env python3

import re
import sys

def is_valid(password):
    numeric = list(map(ord, password))

    return (
        # Include an increasing subsequence
        any(
            numeric[i] + 2 == numeric[i + 1] + 1 == numeric[i + 2]
            for i in range(len(password) - 2)
        )
        # May not contain i, o, or l
        and not any(c in password for c in 'iol')
        # Must have at least two different pairs
        and len(set(re.findall(r'(.)\1', password))) >= 2
    )

def increment(password):
    numeric = list(map(ord, password))
    index = -1

    while True:
        numeric[index] += 1
        if numeric[index] <= ord('z'):
            break
        else:
            numeric[index] = ord('a')
            index -= 1

    return ''.join(map(chr, numeric))

def next_valid(password):
    while True:
        password = increment(password)
        if is_valid(password):
            return password

print(next_valid(sys.argv[1]))
