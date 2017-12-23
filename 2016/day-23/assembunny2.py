#!/usr/bin/env python3

import argparse
import fileinput
import logging
import re

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*')
parser.add_argument('--registers', nargs = '+', default = [])
parser.add_argument('--optimize', action = 'store_true')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

class APC(object):
    def __init__(self, code, registers):
        self._code = code
        self._pc = 0
        self._registers = {k: registers.get(k, 0) for k in 'abcd'}

    def __repr__(self):
        return 'APC<{}, {}, {}>'.format(
            id(self),
            self._pc,
            self._registers,
        )

    def optimize(self):
        '''Apply a few hand rolled optimizations to the code.'''

        code = '\n'.join(' '.join(line) for line in self._code)

        replacements = [
            (   # Multiplication
                r'inc ([a-d])\ndec ([a-d])\njnz \2 -2\ndec ([a-d])\njnz \3 -5',
                r'mul \2 \3 \1\ncopy 0 \2\ncopy 0 \3\nnop\nnop',
            ),
            (   # Addition (v1)
                r'inc ([a-d])\ndec ([a-d])\njnz \2 -2',
                r'add \1 \2 \1\ncopy 0 \2\nnop',
            ),
            (   # Addition (v2)
                r'dec ([a-d])\ninc ([a-d])\njnz \1 -2',
                r'add \1 \2 \2\ncopy 0 \1\nnop',
            ),
        ]

        for pattern, replacement in replacements:
            code = re.sub(pattern, replacement, code)

        logging.info('Optimized code:\n{}'.format(code))

        self._code = [line.split() for line in code.split('\n')]

    def run(self):
        def val(x):
            try:
                return int(x)
            except:
                return self._registers[x]

        while True:
            if not (0 <= self._pc < len(self._code)):
                break

            cmd, *args = self._code[self._pc]
            logging.info('{} running {}({})'.format(self, cmd, args))

            if cmd == 'cpy':
                self._registers[args[1]] = val(args[0])

            elif cmd == 'inc':
                self._registers[args[0]] += 1

            elif cmd == 'dec':
                self._registers[args[0]] -= 1

            elif cmd == 'jnz':
                if val(args[0]) != 0:
                    self._pc += val(args[1]) - 1

            elif cmd == 'tgl':
                target = self._pc + val(args[0])
                if 0 <= target < len(self._code):
                    old_cmd, *old_args = self._code[target]

                    if len(old_args) == 1:
                        new_cmd = 'dec' if old_cmd == 'inc' else 'inc'
                    elif len(old_args) == 2:
                        new_cmd = 'cpy' if old_cmd == 'jnz' else 'jnz'

                    self._code[target] = [new_cmd] + old_args

            # Used by optimizations
            elif cmd == 'nop':
                pass

            elif cmd == 'add':
                self._registers[args[2]] = val(args[0]) + val(args[1])

            elif cmd == 'mul':
                self._registers[args[2]] = val(args[0]) * val(args[1])

            self._pc += 1

if __name__ == '__main__':
    instructions = [
        line.strip().split()
        for line in fileinput.input(args.files)
        if line.strip()
    ]

    registers = {
        arg.split('=')[0].strip(): int(arg.split('=')[1].strip())
        for arg in args.registers
    }

    apc = APC(instructions, registers)
    if args.optimize:
        apc.optimize()

    print('Initial:', apc)
    apc.run()
    print('Final:', apc)
