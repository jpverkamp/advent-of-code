#!/usr/bin/env python3

import collections
import functools

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--sort-by', required = True, choices = ('strength', 'length'))

Bridge = collections.namedtuple('Bridge', 'strength length components')

components = frozenset(
    tuple(map(int, line.split('/')))
    for line in lib.input()
)

@functools.lru_cache(None)
def bridges(connector, components):
    '''Return all bridges that can be constructed from this point.'''

    for component in components:
        x, y = component
        if x == connector or y == connector:
            for strength, length, bridge in bridges(y if x == connector else x, components - {component}):
                yield Bridge(
                    strength + x + y,
                    length + 1,
                    [(x, y)] + bridge
                )

    yield Bridge(0, 0, [])

print(max(
    bridges(0, components),
    key = lambda bridge: (getattr(bridge, lib.param('sort_by')), bridge)
))
