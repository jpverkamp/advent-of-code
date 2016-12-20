#!/usr/bin/env python3

import argparse
import regex as re

parser = argparse.ArgumentParser()
parser.add_argument('input')
args = parser.parse_args()

tls_valid = 0
ssl_valid = 0

re_abba = re.compile(r'([a-z])((?!\1)[a-z])\2\1')
re_aba = re.compile(r'([a-z])((?!\1)[a-z])\1')

with open(args.input, 'r') as fin:
    for line in fin:
        line = line.strip('\n')

        hypernet_list = []
        def store_and_replace(m):
            hypernet_list.append(m.group(0))
            return '--'

        supernet = re.sub(r'\[[a-z]+\]', store_and_replace, line)
        hypernet = '--'.join(hypernet_list)

        tls = ssl = False

        if re_abba.search(supernet) and not re_abba.search(hypernet):
            tls = True

        for (a, b) in re_aba.findall(supernet, overlapped = True):
            if b + a + b in hypernet:
                ssl = True

        if tls: tls_valid += 1
        if ssl: ssl_valid += 1

print('tls:', tls_valid)
print('ssl:', ssl_valid)
