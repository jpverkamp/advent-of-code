#!/usr/bin/env python3

import argparse
import re

parser = argparse.ArgumentParser()
parser.add_argument('input_file')
args = parser.parse_args()

def generate_checksum(name):
    '''
    Custom checksum function by sorting all characters in the input on a tuple
    of: length (shortest first) and the letter itself for alphabetical sorting.
    '''

    return ''.join(list(sorted(
        set(name) - {'-'},
        key = lambda letter : (
            -name.count(letter),
            letter
        )
    )))[:5]

def decrypt(name, key):
    '''Shift all characters in the name by key positions.'''

    offset = ord('a')
    def shift(c):
        if c == '-':
            return ' '
        else:
            return chr((ord(c) - offset + key) % 26 + offset)

    return ''.join(map(shift, name))

valid_sector_id_sum = 0
potential_north_sectors = []

with open(args.input_file, 'r') as fin:
    for room in fin:
        m = re.match(r'([a-z-]+)-(\d+)\[([a-z]+)\]', room)
        name, sector_id, checksum = m.groups()

        if checksum != generate_checksum(name):
            continue

        valid_sector_id_sum += int(sector_id)

        real_name = decrypt(name, int(sector_id))
        if 'north' in real_name:
            potential_north_sectors.append((real_name, sector_id))

print('sum of valid ids:', valid_sector_id_sum)
for real_name, sector_id in potential_north_sectors:
    print(real_name, '@', sector_id)
