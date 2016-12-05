#!/usr/bin/env python3

import collections
import re
import sys

transitions = collections.defaultdict(set)

reading_transitions = True
for line in sys.stdin:
    line = line.strip()

    if not line:
        reading_transitions = False
    elif reading_transitions:
        src, dst = line.split(' => ')
        transitions[src].add(dst)
    else:
        target = line

def expand_iter(input):
    for src in transitions:
        for dst in transitions[src]:
            for match in re.finditer(src, input):
                yield input[:match.start()] + dst + input[match.end():]

expansions = set(expand_iter(target))

print(len(expansions))
