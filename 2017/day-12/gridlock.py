#!/usr/bin/env python3

import collections
import queue

import sys; sys.path.insert(0, '..'); import lib

nodes = set()
neighbors = collections.defaultdict(set)

for line in lib.input():
    source, destinations = line.split('<->')
    source = int(source.strip())
    nodes.add(source)

    for destination in destinations.strip().split(','):
        destination = int(destination.strip())
        nodes.add(destination)

        neighbors[source].add(destination)
        neighbors[destination].add(source)

def find_group(node):
    '''Yield all nodes that are connected to the given node.'''

    visited = set()
    q = queue.Queue()
    q.put(node)

    while not q.empty():
        node = q.get()

        if node in visited:
            continue
        else:
            visited.add(node)

        yield node

        for neighbor in neighbors[node]:
            q.put(neighbor)

print('the group containing 0 has {} nodes'.format(len(list(find_group(0)))))

visited = set()
groups = []

for node in nodes:
    if node in visited:
        continue

    group = set(find_group(node))
    groups.append(group)
    visited |= group

    lib.log('Found new group: {}', group)

print('there are {} groups'.format(len(groups)))
