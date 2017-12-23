#!/usr/bin/env python3

import pyparsing as pp
import re

import sys; sys.path.insert(0, '..'); import lib

_last_garbage_count = 0

def parse(stream):
    '''Implement a pyparsing parser for the stream format.'''

    global _last_garbage_count
    _last_garbage_count = 0

    group = pp.Forward()

    def count_garbage(s, l, t):
        global _last_garbage_count
        _last_garbage_count += len(re.sub(r'!.', '', t[0][1:-1]))

    garbage = pp.Suppress(pp.Regex(r'<([^!>]|!.)*>').setParseAction(count_garbage))
    data = pp.Regex(r'[^}]+')

    # Using QuotedString (as below) won't tell you how many characters were escaped
    # garbage = pp.Suppress(pp.QuotedString('<', escChar = '!', endQuoteChar='>')

    group << pp.Group(
        pp.Suppress('{')
        + pp.Optional(pp.delimitedList(group | garbage | data))
        + pp.Suppress('}')
    )

    parser = (group | garbage)

    data = parser.parseString(stream)
    if data:
        return data[0]

def count_groups(data):
    '''Count how many groups there are in the given data.'''

    if isinstance(data, pp.ParseResults):
        data = data.asList()

    if isinstance(data, list):
        return 1 + sum(count_groups(el) for el in data)
    else:
        return 0

def score_groups(data, depth = 1):
    '''
    The score of a group is equal to its depth.
    The score of data is equal to the sum of its groups.
    '''

    if isinstance(data, pp.ParseResults):
        data = data.asList()

    if isinstance(data, list):
        return depth + sum(score_groups(el, depth + 1) for el in data)
    else:
        return 0

# Parse input
total_score = 0
total_garbage = 0

for line in lib.input():
    data = parse(line)

    count = count_groups(data)
    score = score_groups(data)
    total_score += score

    total_garbage += _last_garbage_count

    lib.log('input: {}, output: {}, count: {}, score: {}, garbage: {}', line, data, count, score, _last_garbage_count)

print('score: {}, garbage: {}'.format(total_score, total_garbage))
