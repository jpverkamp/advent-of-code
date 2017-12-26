#!/usr/bin/env python3

import queue
import re

import sys; sys.path.insert(0, '..'); import lib

data = {}
points = {}
entry = None

def is_point(c):
    return re.match('[A-Z]', c)

for y, line in enumerate(lib.input()):
    for x, c in enumerate(line):
        if c.strip():
            data[x, y] = c

        if is_point(c):
            points[c] = (x, y)

        if y == 0 and c == '|':
            entry = (x, y)

last_step_count = 0

def path(pt):
    '''Yield all points along a given path.'''

    global last_step_count
    last_step_count = 0

    x, y = pt
    xd, yd = (0, 1)
    steps = 0

    while True:
        pt = x, y = x + xd, y + yd
        c = data.get(pt)
        lib.log(f'At {pt}, facing {(xd, yd)}: {c}')

        last_step_count += 1

        if not c:
            break
        elif is_point(c):
            yield c
        elif c == '+':
            for new_xd, new_yd in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                # Back the way we came
                if new_xd == -xd and new_yd == -yd:
                    continue
                elif not data.get((x + new_xd, y + new_yd)):
                    continue
                else:
                    xd, yd = new_xd, new_yd
                    lib.log(f'Turned towards {(xd, yd)}')
                    break

print(''.join(path(entry)))
print(last_step_count)
