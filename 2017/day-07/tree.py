#!/usr/bin/env python3

import collections
import functools
import re

import sys; sys.path.insert(0, '..'); import lib

names = set()
weight_map = {}
child_map = {}
parent_map = {}

for line in lib.input():
    name, weight, children = re.match(r'(\w+) \((\d+)\)(?: -> (.*))?', line).groups()

    names.add(name)
    weight_map[name] = int(weight)

    if children:
        for child in children.split(', '):
            child_map.setdefault(name, set()).add(child)
            parent_map[child] = name

# The root node is the only one without a parent
for name in names:
    if name not in parent_map:
        root = name

print('root: {}'.format(root), end = '; ')

@functools.lru_cache(None)
def total_weight(node):
    '''Return the weight of this node + the sum of all children.'''

    lib.log('Calculating weight of {}'.format(node))

    return weight_map[node] + sum(total_weight(child) for child in child_map.get(node, []))

def fix_balance(node):
    '''Fix the balance from this node (recursively).'''

    lib.log('Fixing the balance at {}'.format(node))

    # If we have no children, no point in balancing
    if not child_map.get(node):
        return

    # Collect a map of weight to set of children with that weight
    # We're unbalanced if this map has two keys, one with a single value
    weights = collections.defaultdict(set)
    for child in child_map[node]:
        weights[total_weight(child)].add(child)
        yield from fix_balance(child)

    # If we only have a single weight, this node is not unbalanced
    if len(weights) == 1:
        return

    # Otherwise, figure out which node is unbalanced (the single mismatched weight)
    for weight, children in weights.items():
        if len(children) == 1:
            unbalanced_node = list(children)[0]
            unbalanced_weight = weight
        else:
            balanced_weight = weight

    # Balance it
    weight_map[unbalanced_node] += balanced_weight - unbalanced_weight
    yield unbalanced_node, weight_map[unbalanced_node]

for node, new_weight in fix_balance(root):
    print('{} -> {}'.format(node, new_weight))
    break
