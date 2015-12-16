#!/usr/bin/env python3

import hashlib
import sys

def naturals(i = 0):
	while True:
		yield i
		i += 1

def mine(prefix, length):
	for i in naturals(1):
		coin = '{prefix}{suffix}'.format(prefix = prefix, suffix = i).encode('utf-8')
		hash = hashlib.md5(coin).hexdigest()
		if all(c == '0' for c in hash[0:length]):
			return (i, hash)

print(mine(sys.argv[1], 5))