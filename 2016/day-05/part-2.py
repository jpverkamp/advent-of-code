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

password = list('--------')

print('index     hash    password')
for i in naturals():
    hash = md5(args.salt + str(i))
    sys.stdout.write('{: <9} {} {}\r'.format(i, hash[:7], ''.join(password)))
    sys.stdout.flush()

    if hash[:5] == '00000':
        index = hash[5]
        if index not in '01234567':
            continue

        index = int(index)
        if password[index] != '-':
            continue

        password[index] = hash[6]

    if not(any(c == '-' for c in password)):
        break

sys.stdout.write('{: <9} {} {}\n'.format(i, hash[:7], ''.join(password)))
sys.stdout.flush()
