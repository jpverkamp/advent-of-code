#!/usr/bin/env python3

import argparse
import hashlib
import os
import sys

parser = argparse.ArgumentParser()
parser.add_argument('salt')
args = parser.parse_args()

if os.path.isfile(args.salt):
    with open(args.salt, 'r') as fin:
        args.salt = fin.read().strip()

valid_sector_id_sum = 0

def naturals(i = 0):
    while True:
        yield i
        i += 1

def md5(str):
    return hashlib.md5(str.encode()).hexdigest()

easy_password = ''
hard_password = ['-'] * 8

format_string = '{: <10}' * 4

print(format_string.format('index', 'hash', 'easy', 'hard'))
for i in naturals():
    hash = md5(args.salt + str(i))
    print(format_string.format(i, hash[:6], easy_password, ''.join(hard_password)), end = '\r')
    sys.stdout.flush()

    if hash[:5] == '00000':
        if len(easy_password) < 8:
            easy_password += hash[5]

        index = hash[5]
        if index not in '01234567':
            continue

        index = int(index)
        if hard_password[index] != '-':
            continue

        hard_password[index] = hash[6]

    if not(any(c == '-' for c in hard_password)):
        break

print(format_string.format(i, hash[:6], easy_password, ''.join(hard_password)))
