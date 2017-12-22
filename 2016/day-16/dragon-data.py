#!/usr/bin/env python3

import argparse
import logging
import re

parser = argparse.ArgumentParser()
parser.add_argument('--seed', required = True, help = 'Initial value to see the dragon curve with')
parser.add_argument('--length', type = int, required = True, help = 'Size of the disk to fill')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

def dragon(ls, length):
    logging.info('Expanding: {}'.format(ls))

    while len(ls) < length:
        ls = ls + [0] + list(reversed([0 if el else 1 for el in ls]))
        logging.info('Expanded to: {}'.format(ls))

    return ls[:length]

def checksum(ls):
    logging.info('Checksum of: {}'.format(ls))

    if len(ls) % 2 == 0:
        return checksum([(1 if a == b else 0) for (a, b) in zip(ls[0::2], ls[1::2])])
    else:
        return ls

ls = list(map(int, args.seed))
sum = checksum(dragon(ls, args.length))

print(''.join(map(str, sum)))
