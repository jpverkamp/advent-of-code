#!/usr/bin/env python3

import imp

part1 = imp.load_source('part1', 'part-1.py')

if __name__ == '__main__':
    program = part1.read_program()
    output = part1.run(program, a = 1)
    print(output['b'])
