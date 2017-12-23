#!/usr/bin/env python3

import argparse
import itertools
import logging

parser = argparse.ArgumentParser()
parser.add_argument('--size', type = int, required = True, help = 'Size of the table')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

def last_elf_standing(size):
    sad_elves = set()

    # The magic of lazy evaluation...
    elves = filter(lambda e : e not in sad_elves, itertools.cycle(range(1, size + 1)))

    while True:
        happy_elf = next(elves)
        sad_elf = next(elves)
        sad_elves.add(sad_elf)

        # A single elf is happy because they have lots of presents
        # But sad since they have no friends
        if happy_elf == sad_elf:
            return happy_elf

        logging.info('{} took presents from {}, {} elves remain'.format(
            happy_elf,
            sad_elf,
            size - len(sad_elves),
        ))

print('Last elf standing (sitting?): {}'.format(last_elf_standing(args.size)))
