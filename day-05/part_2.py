import re
import sys

def is_nice(word):
	return (
		re.search(r'(..).*\1', word)
		and re.search(r'(.).\1', word)
	)

nice_count = 0
for line in sys.stdin:
	if is_nice(line.strip()):
		nice_count += 1

print(nice_count)
