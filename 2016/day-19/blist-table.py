#!/usr/bin/env python3

import argparse
import blist
import logging
import re

parser = argparse.ArgumentParser()
parser.add_argument('--size', type = int, required = True, help = 'Size of the table')
parser.add_argument('--function', help = '''\
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

if args.function and re.search(r'[^a-z0-9+\-*/ ]', args.function):
    parser.error('Invalid function')

def last_elf_standing(size, function = None):
    index = 0

    logging.info('Building sorted set...')
    elves = blist.blist(range(1, size + 1))
    logging.info('...done')

    while len(elves) > 1:
        happy_elf = elves[index]

        if function:
            sad_elf_index = eval(args.function, globals(), {
                'index': index,
                'size': size,
                'count': len(elves),
            })
        else:
            sad_elf_index = index + 1

        sad_elf_index %= len(elves)

        sad_elf = elves[sad_elf_index]
        del elves[sad_elf_index]

        # If we removed an elf before us, the index doesn't change (the elf at that index does)
        if sad_elf_index >= index:
            index += 1
        index %= len(elves)

        logging.info('{} took presents from {}, {} elves remain'.format(
            happy_elf,
            sad_elf,
            len(elves),
        ))

    return elves[0]

last_elf = last_elf_standing(args.size, args.function)
print('Last elf standing (sitting?): {}'.format(last_elf))
