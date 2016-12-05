#!/usr/bin/env python3

import argparse
import re

from lib import generate_checksum

parser = argparse.ArgumentParser()
parser.add_argument('input_file')
args = parser.parse_args()

valid_sector_id_sum = 0

with open(args.input_file, 'r') as fin:
    for room in fin:
        m = re.match(r'([a-z-]+)-(\d+)\[([a-z]+)\]', room)
        name, sector_id, checksum = m.groups()

        if checksum == generate_checksum(name):
            valid_sector_id_sum += int(sector_id)

print(valid_sector_id_sum)
