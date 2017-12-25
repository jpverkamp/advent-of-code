#!/usr/bin/env python3

import functools
import math

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--programs', default = 'abcdefghijklmnop')
lib.add_argument('--repeat', type = int, default = 1)

data = list(lib.param('programs'))

commands = [
    command
    for line in lib.input()
    for command in line.split(',')
]

@functools.lru_cache(None)
def generate_swaps(iterations):
    '''
    Recursively determine how a list will be mutated for a given number of mutations.
    '''

    # Base case: Don't change for 0 iterations
    if iterations == 0:
        return list(range(len(lib.param('programs')))), {}

    # Base case, manually calculate a single iteration
    elif iterations == 1:
        position_swaps = list(range(len(lib.param('programs'))))
        character_swaps = []

        for command in commands:
            if command.startswith('s'):
                distance = int(command[1:])
                position_swaps = position_swaps[len(position_swaps)-distance:] + position_swaps[:len(position_swaps)-distance]

            elif command.startswith('x'):
                x, y = map(int, command[1:].split('/'))
                position_swaps[x], position_swaps[y] = position_swaps[y], position_swaps[x]

            elif command.startswith('p'):
                a, b = command[1:].split('/')
                character_swaps.append((a, b))

        character_swap_map = {}
        for c in data:
            out = c
            for a, b in character_swaps:
                if out == a:
                    out = b
                elif out == b:
                    out = a
            if c != out:
                character_swap_map[c] = out

        lib.log('Base case (iterations = 1)')
        lib.log('Position swaps: {}', position_swaps)
        lib.log('Character swaps: {}', character_swap_map)

        return position_swaps, character_swap_map

    # Recursive case, split in half and combine
    else:
        i = math.floor(iterations / 2)
        j = math.ceil(iterations / 2)

        position_swaps_i, character_swap_map_i = generate_swaps(i)
        position_swaps_j, character_swap_map_j = generate_swaps(j)

        lib.log('Combining position swaps for {}={} and {}={}', i, position_swaps_i, j, position_swaps_j)

        position_swaps = [
            position_swaps_i[index]
            for index in position_swaps_j
        ]

        lib.log('Combining character swaps for {}={} and {}={}', i, character_swap_map_i, j, character_swap_map_j)

        character_swap_map = {}
        for k in set(character_swap_map_i.keys()) | set(character_swap_map_j.keys()):
            v1 = character_swap_map_i.get(k, k)
            v2 = character_swap_map_j.get(v1, v1)
            if k != v2:
                character_swap_map[k] = v2

        lib.log('Recursive case (iterations = {})'.format(iterations))
        lib.log('Position swaps: {}', position_swaps)
        lib.log('Character swaps: {}', character_swap_map)

        return position_swaps, character_swap_map

def apply_swaps(data, position_swaps, character_swap_map):
    '''
    Apply a pair of swap maps to the data and return the result.
    '''

    return [
        character_swap_map.get(data[index], data[index])
        for index in position_swaps
    ]

position_swaps, character_swap_map = generate_swaps(lib.param('repeat'))
data = apply_swaps(data, position_swaps, character_swap_map)

output = ''.join(data)
print(output)
