#!/usr/bin/env python3

import sys

def look_and_say(seq):
    result = ''
    index = 0
    count = 0
    buffer = None

    for c in seq:
        if c == buffer:
            count += 1
        else:
            if buffer:
                result += '{}{}'.format(count, buffer)
            count = 1
            buffer = c

    result += '{}{}'.format(count, buffer)
    return result

def repeat(f, n, seq):
    for i in range(n):
        seq = f(seq)
    return seq

print(len(repeat(look_and_say, int(sys.argv[2]), sys.argv[1])))
