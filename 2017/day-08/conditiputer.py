#!/usr/bin/env python3

import collections
import operator
import re

import sys; sys.path.insert(0, '..'); import lib

registers = collections.defaultdict(lambda : 0)

conditionals = {
    '<': operator.lt,
    '<=': operator.le,
    '==': operator.eq,
    '!=': operator.ne,
    '>=': operator.ge,
    '>': operator.gt,
}

def val(x):
    try:
        return int(x)
    except:
        return registers[x]

max_register_value = 0
for line in lib.input():
    lib.log('{}, applying {}', dict(registers), line)

    register, mode, value, _, left, op, right = line.split()

    f = conditionals[op]

    if f(val(left), val(right)):
        if mode == 'inc':
            registers[register] += val(value)
        else:
            registers[register] -= val(value)

    max_register_value = max(max_register_value, *registers.values())

lib.log('Final registers: {}', dict(registers))

print('maximums, final: {}, overall: {}'.format(
    max(registers.values()),
    max_register_value,
))
