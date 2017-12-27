#!/usr/bin/env python3

import math

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--iterations', type = int, required = True, help = 'Number of iterations to run the simulation')
lib.add_argument('--render', action = 'store_true', help = 'Render steps for debug mode (slow)')

state = {}

# Load the initial state
data = ''.join(lib.input(include_comments = True))

size = int(math.sqrt(len(data)))
offset = -(size // 2)

for x in range(size):
    for y in range(size):
        if data[y * size + x] == '#':
            infected.add((x + offset, y + offset))

# Helper functions
def render():
    min_y = min(y for x, y in infected)
    max_y = max(y for x, y in infected)
    min_x = min(y for x, y in infected)
    max_x = max(x for x, y in infected)

    return '\n'.join(
        ''.join(
            '*' if (x, y) == location else ('#' if (x, y) in infected else '.')
            for x in range(min_x - 1, max_x + 2)
        )
        for y in range(min_y - 1, max_y + 2)
    )

# Run the simulation
location = (0, 0)
facing = (0, -1)

caused_infection = 0

for tick in range(0, lib.param('iterations')):
    if lib.param('render'):
        lib.log(f'Tick {tick}\n{render()}\n')

    if location not in infected:
        caused_infection += 1

    facing = lib.vector2_rotate(facing, location in infected)
    infected ^= {location}
    location = lib.vector_add(location, facing)

lib.log(f'Final state (tick {tick+1})\n{render()}\n')

print(f'{caused_infection} new infections')
