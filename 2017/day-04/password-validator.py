#!/usr/bin/env python3

import argparse
import fileinput
import logging
import re

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*')
parser.add_argument('--no-anagrams', action = 'store_true')
parser.add_argument('--debug', action = 'store_true')
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

valid_count = 0
total_count = 0

for line in fileinput.input(args.files):
    line = line.strip()
    if not line or line.startswith('#'):
        continue

    total_count += 1

    words = line.split()
    if args.no_anagrams:
        words = [''.join(sorted(word)) for word in words]

    if len(words) == len(set(words)):
        logging.info('{} is valid'.format(line))
        valid_count += 1
    else:
        logging.info('{} is invalid'.format(line))

print(valid_count)
