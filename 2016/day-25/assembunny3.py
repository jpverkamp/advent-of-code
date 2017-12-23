#!/usr/bin/env python3

import argparse
import itertools
import fileinput
import logging
import re

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*')
parser.add_argument('--show_output', action = 'store_true', help = 'Show each output value')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

class APC(object):
    def __init__(self, code, registers):
        self._code = code
        self._pc = 0
        self._registers = {k: registers.get(k, 0) for k in 'abcd'}
        self._output = []

    def __repr__(self):
        return 'APC<{}, {}, {}, {}>'.format(
            id(self),
            self._pc,
            self._registers,
            self._output,
        )

    def output(self):
        '''Return output as a string.'''

        return ''.join(str(el) for el in self._output)

    def run(self):
        def val(x):
            try:
                return int(x)
            except:
                return self._registers[x]

        seen_states = set()

        while True:
            if not (0 <= self._pc < len(self._code)):
                break

            # Automatically halt if we see a state we've seen before
            state = (self._pc, tuple(v for k, v in sorted(self._registers.items())))
            if state in seen_states:
                break
            else:
                seen_states.add(state)

            cmd, *args = self._code[self._pc]

            if cmd == 'cpy':
                self._registers[args[1]] = val(args[0])

            elif cmd == 'inc':
                self._registers[args[0]] += 1

            elif cmd == 'dec':
                self._registers[args[0]] -= 1

            elif cmd == 'jnz':
                if val(args[0]) != 0:
                    self._pc += val(args[1]) - 1

            elif cmd == 'out':
                self._output.append(val(args[0]))

            # Used by optimizations
            elif cmd == 'nop':
                pass

            elif cmd == 'add':
                self._registers[args[2]] = val(args[0]) + val(args[1])

            elif cmd == 'mul':
                self._registers[args[2]] = val(args[0]) * val(args[1])

            self._pc += 1

instructions = [
    line.strip().split()
    for line in fileinput.input(args.files)
    if line.strip()
]

for a_value in itertools.count():
    logging.info('Trying a = {}'.format(a_value))
    apc = APC(instructions, {'a': a_value})

    logging.info('Initial: {}'.format(apc))
    apc.run()
    logging.info('Final: {}'.format(apc))

    if args.show_output:
        print('{:05d} {}'.format(a_value, apc.output()))

    output = apc.output()
    if apc.output() == ('01' * (len(output) // 2)):
        print('Found a repeating signal when a = {}'.format(a_value))
        break
