#!/usr/bin/env python3

import itertools
import pprint

import sys; sys.path.insert(0, '..'); import lib

# Map of (current state, current value, key) -> value
# key is one of value, offset, state
transitions = {}
breakpoint = 0
state = None
pointer = 0
one_bits = set()

for line in lib.input():
    line = line.strip('- ')

    # Parse the argument from the line
    arg = line.split()[-1][:-1]

    if arg == 'steps':
        arg = line.split()[-2]

    try:
        arg = int(arg)
    except:
        pass

    # Store values based on that argument
    if line.startswith('Begin'):
        state = arg
    elif line.startswith('Perform'):
        breakpoint = arg
    elif line.startswith('In'):
        current_state = arg
    elif line.startswith('If'):
        current_value = arg
    elif line.startswith('Write'):
        transitions[current_state, current_value, 'value'] = arg == 1
    elif line.startswith('Move'):
        transitions[current_state, current_value, 'offset'] = 1 if arg == 'right' else -1
    elif line.startswith('Continue'):
        transitions[current_state, current_value, 'state'] = arg

lib.log('{}', pprint.pformat(transitions))

for tick in range(breakpoint):
    value = 1 if pointer in one_bits else 0

    context = ''.join('1' if pointer + i in one_bits else '0' for i in range(-5, 6))
    lib.log(f'{tick}: state={state} pointer={pointer} value={value} context={context}')

    if value and not transitions[state, value, 'value']:
        one_bits.remove(pointer)
    elif not value and transitions[state, value, 'value']:
        one_bits.add(pointer)

    pointer += transitions[state, value, 'offset']
    state = transitions[state, value, 'state']

print(len(one_bits))
