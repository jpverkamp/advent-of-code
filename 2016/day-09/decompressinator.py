#!/usr/bin/env python3

import argparse
import os
import regex as re

parser = argparse.ArgumentParser()
parser.add_argument('input')
args = parser.parse_args()

re_compressed_block = re.compile(r'\((?P<length>\d+)x(?P<count>\d+)\)')

if os.path.exists(args.input):
    with open(args.input, 'r') as fin:
        content = re.sub('\s+', '', fin.read())
else:
    content = args.input

def decompressed_length(content, version):
    index = 0
    output_length = 0

    while True:
        m = re_compressed_block.search(content, pos = index)
        if m:
            # Content before the current block
            output_length += m.start() - index

            # The current block, expand recursively for version 2
            length = int(m.group('length'))
            count = int(m.group('count'))

            if version == 1:
                output_length += length * count
            elif version == 2:
                block = content[m.end() : m.end() + length]
                expanded_block_length = decompressed_length(block, version = 2)
                output_length += expanded_block_length * count

            # Skip past this block for the next iteration
            index = m.end() + length
        else:
            break

    output_length += len(content) - index

    return output_length

print('v1:', decompressed_length(content, 1))
print('v2:', decompressed_length(content, 2))
