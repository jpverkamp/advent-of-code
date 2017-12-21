#!/usr/bin/env python3

import argparse
import hashlib
import logging
import functools
import re

parser = argparse.ArgumentParser()
parser.add_argument('--salt', help = 'The salt value to generate keys from')
parser.add_argument('--count', type = int, default = 50, help = 'The number of keys to find')
parser.add_argument('--stretch', type = int, default = 1, help = 'Use key stretching by repeating the hash this many times')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

def naturals(i = 0):
    while True:
        yield i
        i += 1

@functools.lru_cache(None)
def hash(n, repeat = 1):
    value = '{}{}'.format(args.salt, n)
    for i in range(repeat):
        value = hashlib.md5(value.encode()).hexdigest()
    return value

keys = set()

for i in naturals():
    key = hash(i, args.stretch)

    for triple in re.findall(r'(.)\1\1', key):
        logging.info('Potential key: {} ({}), repeats {}'.format(i, key, triple))
        quintuple = triple * 5

        for j in range(i + 1, i + 1001):
            if quintuple in hash(j, args.stretch):
                logging.info('- Validated')
                keys.add(key)
                break

        # Only consider the first key
        break

    if len(keys) >= args.count:
        break

print('{} keys generated after {} hashes'.format(len(keys), i))
