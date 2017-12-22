#!/usr/bin/env python3

import argparse
import bisect
import fileinput
import logging
import re

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*', help = 'List of IP filters')
parser.add_argument('--range', default = '0-4294967295', help = 'Overall range of possible IPs')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

class IntRange(object):
    '''
    Represents a list of integers.

    Specific values can be allowed (included) / denied (excluded) from the range.
    '''

    def __init__(self, min, max):
        '''Create a new int range with the given values initially allowed.'''

        self._ranges = [(min, max)]

    def __repr__(self):
        '''Pretty print a range (this can get long).'''

        return 'IntRange<{}>'.format(self._ranges)

    def __in__(self, value):
        '''Test if a value is in this int range.'''

        # Slower version
        # return any(lo <= value <= hi for (lo, hi) in self._ranges)

        index = bisect.bisect(self._ranges, (value, value))
        lo, hi = self._ranges[index]
        return lo <= value <= hi

    def __iter__(self):
        '''Return all values in this int range.'''

        for (lo, hi) in self._ranges:
            yield from range(lo, hi + 1)

    def __len__(self):
        '''Return how many values are in this IP range.'''

        return sum(hi - lo + 1 for (lo, hi) in self._ranges)

    def _simplify(self):
        '''Go through current ranges and remove/collapse overlapping ranges.'''

        logging.info('Simplifing')

        i = 0
        while i + 1 < len(self._ranges):
            range1_lo, range1_hi = self._ranges[i]
            range2_lo, range2_hi = self._ranges[i + 1]

            # Only guarantee: lo1 is <= lo2

            # There is an overlap, combine and remove range2
            # Continue without incrementing since another range might be collapsed
            if range2_lo <= range1_hi:
                logging.info('- Combining {} and {}'.format((range1_lo, range1_hi), (range2_lo, range2_hi)))

                self._ranges[i] = (range1_lo, max(range1_hi, range2_hi))
                del self._ranges[i + 1]
                continue

            i += 1

    def allow(self, allow_min, allow_max):
        '''Add a new range of allowed values.'''

        logging.info('Allow {}'.format((allow_min, allow_max)))

        # Insert sorted (using bisect) then simplify
        bisect.insort(self._ranges, (allow_min, allow_max))
        self._simplify()

    def deny(self, deny_min, deny_max):
        '''Remove a range of (possibly) previously allowed values.'''

        logging.info('Deny {}'.format((deny_min, deny_max)))

        i = 0
        while i < len(self._ranges):
            lo, hi = self._ranges[i]

            # Range is completely denied
            if deny_min <= lo <= hi <= deny_max:
                logging.info('- Deleting {}'.format(self._ranges[i]))

                del self._ranges[i]
                continue

            # Denial is completely within the range, split it
            elif lo <= deny_min <= deny_max <= hi:
                logging.info('- Splitting {}'.format(self._ranges[i]))

                del self._ranges[i]
                self._ranges.insert(i, (lo, deny_min - 1))
                self._ranges.insert(i + 1, (deny_max + 1, hi))

            # Partial overlap, adjust the range
            elif lo <= deny_min <= hi:
                logging.info('- Resizing (min) {}'.format(self._ranges[i]))
                self._ranges[i] = (lo, deny_min - 1)

            elif lo <= deny_max <= hi:
                logging.info('- Resizing (max) {}'.format(self._ranges[i]))
                self._ranges[i] = (deny_max + 1, hi)

            i += 1

lo, hi = map(int, args.range.split('-'))
ips = IntRange(lo, hi)
logging.info('Created IP range: {}'.format(ips))

for line in fileinput.input(args.files):
    if line:
        lo, hi = map(int, line.split('-'))
        ips.deny(lo, hi)
        logging.info('Added range to deny list: {}'.format(lo, hi))

logging.info('Final IP range: {}'.format(ips))
for ip in ips:
    print('First allowed IP: {}'.format(ip))
    break

print('Number of allowed IPs: {}'.format(len(ips)))
