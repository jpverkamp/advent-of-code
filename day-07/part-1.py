#!/usr/bin/env python3

import operator
import re
import sys

monops = {
	'NOT': lambda x : ~x & 0xFFFF,
}

binops = {
	'AND': operator.and_,
	'OR': operator.or_,
	'LSHIFT': operator.lshift,
	'RSHIFT': operator.rshift,
}

machine = {}

for line in sys.stdin:
	line = line.strip()

	m = (
		re.match('(\w+) -> (\w+)', line)
		or re.match('(\w+) (\w+) (\w+) -> (\w+)', line)
		or re.match('(\w+) (\w+) -> (\w+)', line)
	).groups()

	machine[m[-1]] = m[:-1]

def evaluate(register_or_value):
	try:
		return int(register_or_value)
	except:
		return run(register_or_value)

def run(register, state = {}):
	if not register in state:
		command = machine[register]

		if len(command) == 1:
			value, = command
			state[register] = evaluate(value)

		elif len(command) == 2:
			monop, input = command
			state[register] = monops[monop](evaluate(input))

		elif len(command) == 3:
			input_1, binop, input_2 = command
			state[register] = binops[binop](evaluate(input_1), evaluate(input_2))

	return state[register]


print(run('a'))