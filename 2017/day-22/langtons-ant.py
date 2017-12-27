#!/usr/bin/env python3

import math

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--iterations', type = int, required = True, help = 'Number of iterations to run the simulation')
lib.add_argument('--render', action = 'store_true', help = 'Render steps for debug mode (slow)')

lib.add_argument('--mode', default = 'default', help = '''\
Program for the ant, a space delimited list of:
- Input symbol (any symbol may be used, defaults are #.)
- One > for each right turn (so >>> to turn left)
- Output symbol (same alphabet as input)

Predefined modes:
- default = #>. .>>>#
- evolved = .>>>W W# #>F F>>.

Any states not defined default to go straight and leave the symbol alone.''')

# Load the initial state, assuming a square; recenter on the center of the data
# . is assumed to be the default state and not stored
data = ''.join(lib.input(include_comments = True))

size = int(math.sqrt(len(data)))
offset = -(size // 2)

state = {}

for x in range(size):
    for y in range(size):
        if data[y * size + x] != '.':
            state[x + offset, y + offset] = data[y * size + x]

# Load the transition table
# Map of input -> (output, # turns clockwise)
predefined_transitions = {
    'default': '#>. .>>>#',
    'evolved': '.>>>W W# #>F F>>.',
}
mode = lib.param('mode')
mode = predefined_transitions.get(mode, mode)

transitions = {
    rule[0]: (rule[-1], len(rule) - 2)
    for rule in mode.split()
}

lib.log('Transition table: {}', transitions)

# Run the simulation
location = (0, 0)
facing = (0, -1)

def render():
    location_x, location_y = location

    min_x = min(location_x, min(y for x, y in state))
    max_x = max(location_x, max(x for x, y in state))
    min_y = min(location_y, min(y for x, y in state))
    max_y = max(location_y, max(y for x, y in state))

    facing_icon = {
        (0, -1): '^',
        (0, 1): 'v',
        (-1, 0): '<',
        (1, 0): '>',
    }

    return '\n'.join(
        ''.join(
            facing_icon[facing] if (x, y) == location else state.get((x, y), '.')
            for x in range(min_x - 1, max_x + 2)
        )
        for y in range(min_y - 1, max_y + 2)
    )

caused_infection = 0

for tick in range(0, lib.param('iterations')):
    if lib.param('render'):
        lib.log(f'Tick {tick}\n{render()}')

    current = state.get(location, '.')
    output, turns = transitions.get(current, (current, 0))

    lib.log(f'current: {current}, next: {output}, turn {turns} times')

    if output == '.':
        del state[location]
    else:
        state[location] = output

    facing = lib.vector2_rotate(facing, turns)
    location = lib.vector_add(location, facing)

    if output == '#':
        caused_infection += 1

    lib.log('')

    # Old, elegant code when infected was a set
    # facing = lib.vector2_rotate(facing, location in infected)
    # infected ^= {location}
    # location = lib.vector_add(location, facing)

lib.log(f'Final state (tick {tick+1})\n{render()}\n')

print(f'{caused_infection} new infections')
