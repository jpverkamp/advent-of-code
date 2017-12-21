#!/usr/bin/env python3

import argparse
import fileinput
import logging

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*')
parser.add_argument('--registers', nargs = '+', default = [])
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

    def run(self):
        def val(x):
            try:
                return int(x)
            except:
                return self._registers[x]

        try:
            while True:
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

                self._pc += 1

        except Exception as ex:
            logging.info('Exception: {}'.format(ex))
            self.exception = ex

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

    print('Initial:', apc)
    apc.run()
    print('Final:', apc)
