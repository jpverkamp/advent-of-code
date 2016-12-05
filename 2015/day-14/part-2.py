#!/usr/bin/env python3

import re
import sys

simulation_time = int(sys.argv[1])
data = {}

for line in sys.stdin:
    name, speed, time, rest = re.match(
        r'(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.',
        line
    ).groups()

    data[name] = {
        'speed': int(speed),
        'running_time': int(time),
        'resting_time': int(rest),
        'distance': 0,
        'resting': False,
        'timer': int(time),
        'score': 0,
    }

for tick in range(1, simulation_time + 1):
    for key in data:
        if data[key]['timer'] == 0:
            data[key]['resting'] = not data[key]['resting']
            data[key]['timer'] = (
                data[key]['resting_time']
                if data[key]['resting']
                else data[key]['running_time']
            )

        if not data[key]['resting']:
            data[key]['distance'] += data[key]['speed']

        data[key]['timer'] -= 1

    max_distance, max_name = max((data[key]['distance'], key) for key in data)

    for key in data:
        if data[key]['distance'] == max_distance:
            data[key]['score'] += 1

print(max(
    (data[key]['score'], key)
    for key in data
))
