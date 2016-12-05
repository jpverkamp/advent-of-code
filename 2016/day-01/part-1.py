#!/usr/bin/env python3

import sys

location = 0+0j
facing = 1+0j

rotations = {'R': 0+1j, 'L': 0-1j}

commands = sys.stdin.read().split(', ')
for command in commands:
    facing *= rotations[command[0]]
    location += facing * int(command[1:])

print(abs(int(location.real)) + abs(int(location.imag)))
