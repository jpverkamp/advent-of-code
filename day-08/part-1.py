#!/usr/bin/env python3

import ast
import sys

memory_count = 0
raw_count = 0

for line in sys.stdin:
	raw = line.strip()
	parsed = ast.literal_eval(raw) # This is probably cheating

	raw_count += len(raw)
	memory_count += len(parsed)

print(raw_count - memory_count)