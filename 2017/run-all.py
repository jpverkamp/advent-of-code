#!/usr/bin/env python3

import subprocess
import time
import yaml

with open('test-cases.yaml') as fin:
	data = yaml.load(fin)

for folder in sorted(data):
	for command in data[folder]:
		start = time.time()
		output = subprocess.check_output(command, shell = True, cwd = folder).decode().strip()
		end = time.time()

		print(
			folder,
			command,
			end - start,
			output,
			sep = '\t',
		)
