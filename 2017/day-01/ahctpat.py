#!/usr/bin/env python3

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--function', default = '1', help = 'Function for offset, variables: size')

for line in lib.input():
    total = 0

    offset = lib.math(lib.param('function'), {'size': len(line)})

    for c1, c2 in zip(line, line[offset:] + line[:offset]):
        if c1 == c2:
            total += int(c1)

    print(total)
