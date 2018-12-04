#!/usr/bin/env python3

import collections
import fileinput
import re

RE_REGION = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')

Region = collections.namedtuple(
    'Region',
    ['id', 'left', 'top', 'width', 'height'],
)

all_regions = []
for line in fileinput.input():
    id, left, top, width, height = RE_REGION.match(line).groups()
    all_regions.append(Region(
        int(id),
        int(left),
        int(top),
        int(width),
        int(height),
    ))
