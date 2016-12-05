#!/usr/bin/env python3

import collections
import queue
import re
import sys

transitions = {}

reading_transitions = True
for line in sys.stdin:
    line = line.strip()

    if not line:
        reading_transitions = False
    elif reading_transitions:
        src, dst = line.split(' => ')
        transitions[dst] = src
    else:
        target = line

def build_iter(input):
    for dst in transitions:
        src = transitions[dst]
        for match in re.finditer(dst, input):
            yield input[:match.start()] + src + input[match.end():]

q = queue.PriorityQueue()
q.put((len(target), 0, target))

while True:
    length, iterations, current = q.get()

    if current == 'e':
        break

    for precursor in build_iter(current):
        q.put((len(precursor), iterations + 1, precursor))

print(iterations)
