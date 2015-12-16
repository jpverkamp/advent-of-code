#!/usr/bin/env python3

import sys

data = sys.stdin.read()
print(data.count('(') - data.count(')'))