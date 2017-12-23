#!/usr/bin/env python3

import argparse
import itertools
import logging
import re

parser = argparse.ArgumentParser()
parser.add_argument('--size', type = int, required = True, help = 'Size of the table')
parser.add_argument('--function', default = 'index + 1', help = '''\
The next elf to remove, variables available:
- number (of the current elf)
- index (of the current elf in the remaining elves)
- size (of the original table)
- count (the number of elves left)
''')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

if re.search(r'[^a-z0-9+\-*/ ]', args.function):
    parser.error('Invalid function')

class Table(object):
    def __init__(self, size, function):
        self._index = 0
        self._size = size
        self._elves = list(range(1, size + 1))
        self._function = function

    def remove_one(self):
        if len(self._elves) <= 1:
            raise Exception('Last elf standing')

        happy_elf = self._elves[self._index]

        to_remove = eval(self._function, globals(), {
            'index': self._index,
            'size': self._size,
            'count': len(self._elves),
        })
        to_remove %= len(self._elves)
        sad_elf = self._elves[to_remove]

        logging.info('{} (index = {}), took presents from {} (index = {}), {} left ({})'.format(
            happy_elf,
            self._index,
            sad_elf,
            to_remove,
            len(self._elves) - 1,
            str(self._elves)[:60]
        ))

        self._elves.pop(to_remove)
        self._index += 1
        if self._index > len(self._elves):
            self._index = 0

    def last_elf_standing(self):
        while len(self._elves) > 1:
            self.remove_one()
        return self._elves[0]

table = Table(args.size, args.function)
last_elf = table.last_elf_standing()
print('Last elf standing (sitting?): {}'.format(last_elf))
