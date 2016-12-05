#!/usr/bin/env python3

import argparse
import re

from lib import generate_checksum, decrypt

parser = argparse.ArgumentParser()
parser.add_argument('input_file')
args = parser.parse_args()

with open(args.input_file, 'r') as fin:
    for room in fin:
        m = re.match(r'([a-z-]+)-(\d+)\[([a-z]+)\]', room)
        name, sector_id, checksum = m.groups()

        if checksum != generate_checksum(name):
            continue

        real_name = decrypt(name, int(sector_id))

        if 'north' in real_name:
            print(real_name, sector_id)
