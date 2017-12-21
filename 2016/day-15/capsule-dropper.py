#!/usr/bin/env python3

import argparse
import fileinput
import logging
import re

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*', help = 'Input files')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

discs = []

for line in fileinput.input(args.files):
    if not line.strip() or line.startswith('#'):
        continue

    m = re.match(r'Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).', line)
    index, count, current = m.groups()
    discs.append((int(index), int(count), int(current)))

def naturals(i = 0):
    while True:
        yield i
        i += 1

def filtered(stream, filter):
    for el in stream:
        if filter(el):
            yield(el)

def solve():
    for button_press in naturals():
        logging.info('Trying button press at t = {}'.format(button_press))

        success = True

        for (index, count, current) in discs:
            if (button_press + index + current) % count == 0:
                logging.info('- Passes disc {}'.format(index))
            else:
                logging.info('- Hits disc {}'.format(index))
                success = False
                break

        if success:
            return button_press

print('Press the button at t = {}'.format(solve()))
