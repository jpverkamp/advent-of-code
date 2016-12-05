#!/usr/bin/env python3

import sys

location = 0+0j
facing = 1+0j
visited = {0+0j}

rotations = {'R': 0+1j, 'L': 0-1j}

commands = sys.stdin.read().split(', ')
for command in commands:
    facing *= rotations[command[0]]

    for step in range(int(command[1:])):
        location += facing

        if location in visited:
            print(abs(int(location.real)) + abs(int(location.imag)))
            sys.exit(0)

        visited.add(location)
