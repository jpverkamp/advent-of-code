#!/usr/bin/env python3

import queue

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--seeds', nargs = 2, type = int, required = True)
lib.add_argument('--factors', nargs = 2, type = int, default = (16807, 48271))
lib.add_argument('--modulus', type = int, default = 2147483647)
lib.add_argument('--mask', type = int, default = 0xffff)
lib.add_argument('--pairs', type = int, default = 40000000)
lib.add_argument('--filters', nargs = 2, type = int, default = (1, 1))


def make_generator(value, factor, modulus, multiple_filter):
    while True:
        value = value * factor % modulus

        if value % multiple_filter == 0:
            yield value

generator_a, generator_b = [
    make_generator(lib.param('seeds')[i], lib.param('factors')[i], lib.param('modulus'), lib.param('filters')[i])
    for i in (0, 1)
]

matching_masks = 0

for i, a, b in zip(range(lib.param('pairs')), generator_a, generator_b):
    masked_a = a & lib.param('mask')
    masked_b = b & lib.param('mask')

    if masked_a == masked_b:
        matching_masks += 1
        lib.log(f'index {i} ({a} and {b}) match, {matching_masks} so far')

print(f'{matching_masks} match')
