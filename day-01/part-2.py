#!/usr/bin/env python3

import sys

floor = 0
for index, char in enumerate(sys.stdin.read()):
	floor += (1 if char == '(' else -1)
	if floor < 0:
		print(index)
		sys.exit(0)
