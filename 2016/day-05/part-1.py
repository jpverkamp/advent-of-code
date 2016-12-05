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

password = ''

print('index     hash   password')
for i in naturals():
    hash = md5(args.salt + str(i))
    sys.stdout.write('{: <9} {} {}\r'.format(i, hash[:6], password))
    sys.stdout.flush()

    if hash[:5] == '00000':
        password += hash[5]

    if len(password) == 8:
        break

sys.stdout.write('{: <9} {} {}\n'.format(i, hash[:6], password))
sys.stdout.flush()
