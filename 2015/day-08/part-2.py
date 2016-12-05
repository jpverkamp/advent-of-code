#!/usr/bin/env python3

import re
import sys

raw_count = 0
encoded_count = 0

for line in sys.stdin:
	raw = line.strip()
	encoded = re.sub(r'(["\\])', r'\\\1', raw)
	print(raw, encoded)
	raw_count += len(raw)
	encoded_count += len(encoded) + 2 # Quotes are not included

print(encoded_count - raw_count)
