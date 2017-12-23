#!/usr/bin/env python3

import subprocess
import sys
import time
import yaml

with open('test-cases.yaml') as fin:
	data = yaml.load(fin)

for folder in sorted(data):
	if len(sys.argv) > 1 and folder not in sys.argv[1:]:
		continue

	for command in data[folder]:
		start = time.time()
		output = subprocess.check_output(command, shell = True, cwd = folder).decode().strip()
		end = time.time()

		print(
			folder,
			command,
			end - start,
			output.replace('\n', '; '),
			sep = '\t',
		)
