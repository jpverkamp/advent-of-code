#!/usr/bin/env python3

import itertools
import re

import sys; sys.path.insert(0, '..'); import lib

offsets = list(map(int, lib.input()))
pc = 0

for step in itertools.count():
    lib.log('pc: {}, offsets: {}', pc, offsets)

    if not 0 <= pc < len(offsets):
        break

    offset = offsets[pc]

    if lib.part(2) and offset >= 3:
        offsets[pc] -= 1
    else:
        offsets[pc] += 1

    pc += offset

print(step)
