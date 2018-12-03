#!/usr/bin/env python3

import fileinput

sum = 0
for line in fileinput.input():
    sum += int(line.strip())

print(sum)
