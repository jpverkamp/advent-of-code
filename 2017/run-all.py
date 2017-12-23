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

		print('{day}, {cmd}, {time:02f}, {out}'.format(
			day = folder,
			cmd = command,
			time = end - start,
			out = ('...\n' + output) if '\n' in output else output
		))
