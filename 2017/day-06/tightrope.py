#!/usr/bin/env python3

import itertools

import sys; sys.path.insert(0, '..'); import lib

blocks = tuple(
    int(node)
    for line in lib.input()
    for node in line.split()
)

def balance(blocks):
    to_distribute = max(*blocks)
    index = blocks.index(to_distribute)

    for_each = to_distribute // len(blocks)
    left_over = to_distribute - (for_each * len(blocks))
    give_left_over_to = {i % len(blocks) for i in range(index + 1, index + 1 + left_over)}

    lib.log('Balancing {}, max is {} @ {}, {} to each + {} extra', blocks, to_distribute, index, for_each, left_over)

    return tuple(
        (
            (amount if i != index else 0)
            + for_each
            + (1 if i in give_left_over_to else 0)
        )
        for i, amount in enumerate(blocks)
    )

seen = {}
for cycle in itertools.count():
    if blocks in seen:
        break
    else:
        seen[blocks] = cycle

    blocks = balance(blocks)

print(cycle, cycle - seen[blocks])
