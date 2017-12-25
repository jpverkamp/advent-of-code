#!/usr/bin/env python3

import blist

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--step', required = True, type = int, help = 'Number of steps')
lib.add_argument('--values', required = True, type = int, help = 'Number of values to insert')

step = lib.param('step')

index_of_zero = 0
after_zero = None
current_position = 0

for i in range(1, 1 + lib.param('values')):
    current_position = (current_position + step + 1) % i
    lib.log(f'Inserting {i} at {current_position}')

    if current_position < index_of_zero:
        index_of_zero += 1
    elif current_position == index_of_zero:
        after_zero = i

print(after_zero)
