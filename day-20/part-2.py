#!/usr/bin/env python3

import math
import numpy
import sys

target = int(sys.argv[1])

presents = numpy.zeros(target)

for i in range(1, target):
    presents[i:i*50:i] += 11 * i

for i in range(len(presents)):
    if presents[i] >= target:
        print(i)
        sys.exit(0)
