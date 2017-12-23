#!/usr/bin/env python3

import functools
import math
import operator
import sys

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--marks', type = int, required = True, help = 'The number of marks on the string')
lib.add_argument('--rounds', type = int, default = 1, help = 'The number of rounds to rotate the key')
lib.add_argument('--ascii-key', nargs = '?', const = True, help = 'Interpret lengths as ASCII instead of the default')
lib.add_argument('--additional-key-bytes', type = int, nargs = '+', help = 'Additional args to add to the end of the length list')

if lib.param('ascii_key'):
    if isinstance(lib.param('ascii_key'), str):
        lengths = [ord(c) for c in lib.param('ascii_key')]
    else:
        lengths = [ord(c) for line in lib.input() for c in line]
else:
    lengths = [int(el) for line in lib.input() for el in line.split(',')]

if lib.param('additional_key_bytes'):
    lengths += lib.param('additional_key_bytes')

lib.log('Lengths: {}', lengths)

rope = list(range(lib.param('marks')))

def twist(rope, lengths, rounds = 1):
    '''Twist the rope based on lengths; repeating a given number of times.'''

    origin = 0
    skip = -1

    for round in range(rounds):
        for length in lengths:
            skip += 1
            skip %= len(rope)

            lib.log('Rope: {}, about to twist {} and skip {}', rope, length, skip)

            # Reverse the first n elements; move the current position forward over that length
            rope = rope[length:] + list(reversed(rope[:length]))

            # Move the current position forward by the skip size
            rope = rope[skip:] + rope[:skip]

            # Keep track of how much we rotatedso we can rotate it back
            origin -= length + skip

    # Rotate it back so the original 'first' position is first again
    origin %= len(rope)
    rope = rope[origin:] + rope[:origin]

    return rope

rope = twist(rope, lengths, lib.param('rounds'))

lib.log('Final rope: {}', rope)
print(rope[0] * rope[1])

if math.sqrt(lib.param('marks')) != int(math.sqrt(lib.param('marks'))):
    print('Cannot generate a hash; the number of marks must be a perfect square')
    sys.exit(0)

root_length = int(math.sqrt(lib.param('marks')))

sparse_hash = rope
dense_hash = [
    functools.reduce(operator.xor, sparse_hash[i : i+root_length])
    for i in range(0, lib.param('marks'), root_length)
]
hex_hash = ''.join(f'{i:02x}' for i in dense_hash)

print(hex_hash)
