#!/usr/bin/env python3

import itertools
import fileinput

sum = 0
seen = set()

for line in itertools.cycle(fileinput.input()):
    sum += int(line.strip())
    if sum in seen:
        break
    seen.add(sum)

print(sum)
