#!/usr/bin/env python3

import sys; sys.path.insert(0, '..'); import lib

data = [
    [int(el) for el in row.split()]
    for row in lib.input()
]

# Part 1: Sum of differences between largest and smallest in each row
if lib.part(1):
    print(sum(
        max(*row) - min(*row)
        for row in data
    ))

# Part 2: Result of dividing the only two evenly divisble numbers in a row
if lib.part(2):
    print(sum(
        a // b
        for row in data
        for a in row
        for b in row
        if a != b and a % b == 0
    ))
