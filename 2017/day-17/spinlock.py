#!/usr/bin/env python3

import blist

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--step', required = True, type = int, help = 'Number of steps')
lib.add_argument('--values', required = True, type = int, help = 'Number of values to insert')
lib.add_argument('--after', required = True, type = int, help = 'Print the value after this')

data = blist.blist([0])
step = lib.param('step')

current_position = 0
for i in range(1, 1 + lib.param('values')):
    current_position = (current_position + step + 1) % len(data)
    lib.log(f'Inserting {i} at {current_position} into {data}')
    data.insert(current_position, i)

index_of_after = data.index(lib.param('after'))
lib.log('Index of {}: {}'.format(lib.param('after'), index_of_after))
lib.log('Context: {}'.format(data[index_of_after-3 : index_of_after+4]))

print(data[(data.index(lib.param('after')) + 1) % len(data)])
