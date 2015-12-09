#!/usr/bin/env python3

import re
import sys

memory_count = 0
raw_count = 0

patterns = [
	(r'\\"', '"'),
	(r'\\\\', r'\\'),
	(r'\\x(\d\d)', chr),
	(r'^"(.*)"$', r'\1'),
]

for line in sys.stdin:
	parsed = raw = line.strip()
	for src, dst in patterns:
		parsed = re.sub(src, dst, parsed)

	print(raw, parsed)

	raw_count += len(raw)
	memory_count += len(parsed)

print(raw_count - memory_count)