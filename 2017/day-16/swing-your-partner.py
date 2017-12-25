#!/usr/bin/env python3

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--programs', default = 'abcdefghijklmnop')
lib.add_argument('--repeat', type = int, default = 1)

data = list(lib.param('programs'))

# Pre-calculate a list of positions and letters that need swapped
position_swaps = list(range(len(data)))
character_swaps = []

for line in lib.input():
    for command in line.split(','):
        lib.log(f'{"".join(data)}, applying {command}')

        # Spin, written sX, makes X programs move from the end to the front, but maintain their order otherwise.
        if command.startswith('s'):
            distance = int(command[1:])
            data = data[len(data)-distance:] + data[:len(data)-distance]
            position_swaps = position_swaps[len(data)-distance:] + position_swaps[:len(data)-distance]

        # Exchange, written xA/B, makes the programs at positions A and B swap places.
        elif command.startswith('x'):
            x, y = map(int, command[1:].split('/'))
            data[x], data[y] = data[y], data[x]
            position_swaps[x], position_swaps[y] = position_swaps[y], position_swaps[x]

        # Partner, written pA/B, makes the programs named A and B swap places.
        elif command.startswith('p'):
            a, b = command[1:].split('/')
            x = data.index(a)
            y = data.index(b)
            data[x], data[y] = data[y], data[x]
            character_swaps.append((a, b))

# Parse through all swaps (in order) to figure out what each character ends up as
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

lib.log('Position swaps: {}', position_swaps)
lib.log('Character swaps: {}', character_swap_map)

import tqdm
for round in tqdm.tqdm(range(2, 1 + lib.param('repeat'))):
    lib.log(f'{"".join(data)}, applying swaps for round {round}')

    data = [
        character_swap_map.get(data[index], data[index])
        for index in position_swaps
    ]

output = ''.join(data)
print(output)
