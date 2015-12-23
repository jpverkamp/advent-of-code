#!/usr/bin/env python3

import re
import sys

def read_program():
    return [
        tuple(re.split('[, ]+', line.strip()))
        for line in sys.stdin
    ]

def run(program, **initial_state):
    pc = 0
    registers = {'a': 0, 'b': 0}
    registers.update(initial_state)

    while True:
        op = program[pc][0]
        args = program[pc][1:]

        if op == 'hlf':
            registers[args[0]] //= 2
            pc += 1
        elif op == 'tpl':
            registers[args[0]] *= 3
            pc += 1
        elif op == 'inc':
            registers[args[0]] += 1
            pc += 1
        elif op == 'jmp':
            pc += int(args[0])
        elif op == 'jie':
            if registers[args[0]] % 2 == 0:
                pc += int(args[1])
            else:
                pc += 1
        elif op == 'jio':
            if registers[args[0]] == 1:
                pc += int(args[1])
            else:
                pc += 1

        if not (0 <= pc < len(program)):
            break

    return registers

if __name__ == '__main__':
    program = read_program()
    output = run(program)
    print(output['b'])
