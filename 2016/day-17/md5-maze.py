#!/usr/bin/env python3

import argparse
import hashlib
import logging
import queue
import re
import sys

parser = argparse.ArgumentParser()
parser.add_argument('--password', required = True, help = 'Password to use as the base of the hash')
parser.add_argument('--debug', action = 'store_true', default = False)
parser.add_argument('--mode', default = 'min', choices = ('min', 'max'), help = 'Find the minimum/maximum path through the maze')
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

# State is a a list of UDLR (up down left right)
# Start is 0, 0; x is left to right, y is top to bottom

# Moving in the given direction adds this to the initial position
offset = {
    'U': 0-1j,
    'D': 0+1j,
    'L': -1,
    'R': +1,
}

# Order of the first four characters in the hash
order = 'UDLR'

def location(path):
    return sum(offset[char] for char in path)

def solved(path):
    return location(path) == 3+3j

def truncate(data, length):
    if len(data) > length:
        return data[:length-3] + '...'
    else:
        return data

def moves(path, password):
    '''Yield the possible moves from the current and path.'''

    current = location(path)

    hash = hashlib.md5((password + path).encode()).hexdigest()
    for offset_char, hash_char in zip(order, hash):
        next = current + offset[offset_char]
        if hash_char in 'bcdef' and 0 <= next.real < 4 and 0 <= next.imag < 4:
            yield offset_char

def solve(password, mode = 'return'):
    '''
    Find the shortest path through the maze.

    Mode can be:
        return: return the first = shortest path
        yield: yield all paths
    '''

    q = queue.Queue()
    q.put('')

    while not q.empty():
        path = q.get()
        # logging.info('{}, ~{} in queue'.format(truncate(path, 60), q.qsize()))

        if solved(path):
            if mode == 'return':
                return path
            elif mode == 'yield':
                yield path
                continue # Don't look for solutions that hit the end and come back

        for move in moves(path, password):
            q.put(path + move)

    if mode == 'return':
        raise Exception('No solution')

def list_all_recursive(password):
    '''Yield all paths through the maze using a recursive solution.'''

    # This is a bad idea
    sys.setrecursionlimit(10000)

    def generate(path):
        logging.info(path)

        if solved(path):
            yield path

        for move in moves(path, password):
            yield from generate(path + move)

    yield from generate('')

if args.mode == 'min':
    print(solve(args.password))

elif args.mode == 'max':
    # https://en.wikipedia.org/wiki/Longest_path_problem

    best_length, best_solution = 0, None
    for solution in solve(args.password, mode = 'yield'):
        if len(solution) > best_length:
            best_length = len(solution)
            best_solution = solution
            logging.info('New best: {} steps, {}'.format(best_length, truncate(best_solution, 60)))

    print('{}\n{} steps'.format(best_solution, best_length))
