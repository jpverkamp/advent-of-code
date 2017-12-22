#!/usr/bin/env python3

import argparse
import fileinput
import functools
import logging
import re

parser = argparse.ArgumentParser()
parser.add_argument('input', help = 'Input string to scramble')
parser.add_argument('files', nargs = '*', help = 'List of filters to apply')
parser.add_argument('--steps', action = 'store_true', help = 'Print each step')
parser.add_argument('--invert', action = 'store_true', help = 'Run the machine in reverse')
parser.add_argument('--debug', action = 'store_true')
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

functions = []

def register(regex):
    '''Register a function as a command in this simple virtual machine we are building.'''

    def outer(f):
        @functools.wraps(f)
        def inner(value, *args, **kwargs):
            new_value = f(value, *args, **kwargs)
            return new_value or value

        functions.append((re.compile(regex), inner))

        return inner
    return outer

def apply(value, command):
    '''Apply a command to the given value (look for a matching regex).'''

    def guess_type(obj):
        try:
            return int(obj)
        except:
            return obj

    logging.info('Applying {} to {}'.format(command, ''.join(value)))
    for regex, function in functions:
        logging.info('- Testing {}'.format(regex))
        m = regex.match(command)
        if m:
            args = [guess_type(arg) for arg in m.groups()]
            kwargs = {k: guess_type(arg) for k, v in m.groupdict().items()}
            logging.info('- Matches, applying function (args = {}, kwargs = {})'.format(args, kwargs))
            return function(value, *args, **kwargs)

    raise Exception('Unknown command: {}'.format(command))

def apply_inverse(value, command):
    '''Apply the inverse of a command for a given value.'''

    return apply(value, 'invert ' + command)

@register(r'invert swap position (\d+) with position (\d+)')
@register(r'swap position (\d+) with position (\d+)')
def swap_indexes(value, x, y):
    value[int(y)], value[int(x)] = value[int(x)], value[int(y)]

@register(r'invert swap letter (\w) with letter (\w)')
@register(r'swap letter (\w) with letter (\w)')
def swap_letters(value, x, y):
    return [
        {x: y, y: x}.get(c, c)
        for c in value
    ]

@register(r'rotate (left|right) (\d+)')
def rotate(value, direction, offset):
    if direction == 'left':
        return value[offset:] + value[:offset]
    else:
        return value[-offset:] + value[:-offset]

@register(r'invert rotate (left|right) (\d+)')
def rotate_inverse(value, direction, offset):
    return rotate(value, 'right' if direction == 'left' else 'left', offset)

@register(r'rotate based on position of letter (\w)')
def rotate_oddly(value, c):
    '''
    rotate based on position of letter X means that the whole string should be
    rotated to the right based on the index of letter X (counting from 0) as
    determined before this instruction does any rotations. Once the index is
    determined, rotate the string to the right one time, plus a number of times
    equal to that index, plus one additional time if the index was at least 4.
    '''

    index = value.index(c)

    value = rotate(value, 'right', 1)
    value = rotate(value, 'right', index)

    if index >= 4:
        value = rotate(value, 'right', 1)

    return value

@register(r'invert rotate based on position of letter (\w)')
def rotate_oddly_but_in_reverse(value, c):
    # This is a hack, but that's a screwy function to invert...

    for offset in range(len(value)):
        test_value = rotate(value, 'left', offset)
        if rotate_oddly(test_value, c) == value:
            return test_value

@register(r'invert reverse positions (\d+) through (\d+)')
@register(r'reverse positions (\d+) through (\d+)')
def reversed_section(value, x, y):
    return value[:x] + list(reversed(value[x:y+1])) + value[y+1:]

@register(r'move position (\d+) to position (\d+)')
def move_character(value, x, y):
    c = value[x]
    value.pop(x)
    value.insert(y, c)

@register(r'invert move position (\d+) to position (\d+)')
def invert_move_character(value, x, y):
    move_character(value, y, x)

value = list(args.input)

commands = list(fileinput.input(args.files))
if args.invert:
    commands = reversed(commands)

for command in commands:
    command = command.strip()
    if args.steps:
        print(''.join(value), '- (inverted)' if args.invert else '-', command, )

    logging.info('=== === ==='.format(command))

    if not command or command.startswith('#'):
        continue
    else:
        if args.invert:
            value = apply_inverse(value, command)
        else:
            value = apply(value, command)

print(''.join(value))
