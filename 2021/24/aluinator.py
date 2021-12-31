import logging
import typer
import time

from collections import *
from dataclasses import dataclass
from typing import *
from pprint import pprint, pformat

app = typer.Typer()


def disableable_cache(x): return x


PRE = '      '
ALL_INPUTS = set(range(1, 10))
OPS = {
    'add': lambda a, b: a + b,
    'mul': lambda a, b: a * b,
    'div': lambda a, b: a // b,
    'mod': lambda a, b: a % b,
    'eql': lambda a, b: 1 if a == b else 0,
    'neq': lambda a, b: 1 if a != b else 0,
}


@app.command()
def run(file: typer.FileText, input: str, quiet: bool = False):
    input_digits = [int(c) for c in input]

    registers = {
        'w': 0,
        'x': 0,
        'y': 0,
        'z': 0,
    }

    for i, line in enumerate(file, 1):
        line = line.strip()
        logging.debug(f'[{i:04d} {registers=}] {line}')

        cmd, *args = line.split()

        if cmd == 'inp':
            a = args[0]
            registers[a] = input_digits[0]
            input_digits = input_digits[1:]

        else:
            a = args[0]
            b = args[1]

            a_val = registers[a]
            b_val = registers[b] if b in registers else int(b)

            r_val = OPS[cmd](a_val, b_val)

            registers[a] = r_val

    if quiet:
        return registers['z']
    else:
        print(registers['z'])
    
@app.command()
def part1(file: typer.FileText):
    
    for i in range(99999999999999, 1, -1):
        input = str(i)
        if '0' in input:
            continue

        file.seek(0)

        result = run(file, input, True)
        if result == 0:
            break

        if input.endswith('9999'):
            logging.info(f'{input} -> {result}')


    print(input)


@app.command()
def solve(file: typer.FileText):

    blocks = []
    zdiv, cx, cy = 0, 0, 0

    logging.info('Finding block parameters')
    for i, line in enumerate(file, 1):
        line = line.strip()
        last = line.split()[-1]
        logging.debug(f'[{i:04d} {zdiv=} {cx=} {cy=}] {line}')

        if line.startswith('inp'):
            if zdiv:
                logging.info(f'Block found: {zdiv=}, {cx=}, {cy=}')
                blocks.append((zdiv, cx, cy))

        elif line.startswith('div z ') and last not in 'wxyz':
            zdiv = int(line.split()[-1])

        elif line.startswith('add x ') and last not in 'wxyz':
            cx = int(line.split()[-1])

        elif line.startswith('add y ') and last not in 'wxyz':
            cy = int(line.split()[-1])

    logging.info(f'Block found: {zdiv=}, {cx=}, {cy=}')
    blocks.append((zdiv, cx, cy))


    logging.info('----- ----- -----')
    logging.info('Generating equations')
    stack = []
    equations = []

    for i, (zdiv, cx, cy) in enumerate(blocks):
        if zdiv == 1:
            stack.append((i, cy))
            logging.info(f'{zdiv:<4d} {cx:<4d} {cy:<4d}  {stack}')
        else:
            j, cy = stack.pop()
            equations.append((i, j, cx + cy))
            logging.info(f'{zdiv:<4d} {cx:<4d} {cy:<4d}  i{i} = i{j} + {cx} + {cy} = i{j} + {cx+cy}')

    logging.info('----- ----- -----')
    logging.info('Solving for minimum/maximum')

    part1 = ['?'] * 14
    part2 = ['?'] * 14

    for i, j, delta in equations:
        i, j = min(i, j), max(i, j)

        if delta <= 0:
            part1[i], part1[j] = '9', str(9 + delta)
            part2[i], part2[j] = str(1 + abs(delta)), '1'

        else:
            part1[i], part1[j] = str(9 - delta), '9'
            part2[i], part2[j] = '1', str(1 + delta)

        equation = f'i{i} = i{j} + {delta}'
        logging.info(f'{equation:<15s} {"".join(part1)} {"".join(part2)}')

    print('part1:', ''.join(part1))
    print('part2:', ''.join(part2))


@app.callback()
def enableFlags(cache: bool = False, debug: bool = False):
    if debug:
        import coloredlogs  # type: ignore
        coloredlogs.install(level=logging.INFO)

    if cache:
        import functools
        global disableable_cache
        disableable_cache = functools.cache


if __name__ == '__main__':
    app()
