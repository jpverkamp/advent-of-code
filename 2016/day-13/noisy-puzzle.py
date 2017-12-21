#!/usr/bin/env python3

import argparse
import fileinput
import logging
import queue
import re

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*')
parser.add_argument('--function', default = 'x*x + 3*x + 2*x*y + y + y*y + z', help = 'A function that determines walls, z is favorite below')
parser.add_argument('--favorite', required = True, help = 'The z parameter to --function')
parser.add_argument('--debug', action = 'store_true', default = False, help = 'Print debug information, slower')

group = parser.add_mutually_exclusive_group(required = True)
group.add_argument('--target', help = 'Return how many steps it tags to ')
group.add_argument('--fill', help = 'Fill out to ')

args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

if re.match(r'[^xy0-9*+]', args.function):
    raise Exception('Invalid function')

def wall(x, y):
    '''Return if there is a wall at a given x,y'''

    z = int(args.favorite)

    if x < 0 or y < 0:
        return True
    else:
        value = eval(args.function)
        binary = bin(value)
        bits = binary.count('1')
        return bits % 2 == 1

def render(points, target = None):
    target_x, target_y = target if target else (10, 10)

    max_x = target_x
    max_y = target_y

    for (x, y) in points:
        max_x = max(max_x, x, target_x)
        max_y = max(max_y, y, target_y)

    max_x += 1
    max_y += 1

    for y in range(max_y + 1):
        for x in range(max_x + 1):
            if (x, y) in points:
                char = 'O'
            elif wall(x, y):
                char = 'X'
            else:
                char = '.'

            print(char, end = '')
        print()

# A state is (x, y, steps)
def solve(start, target = None, fill = None):
    '''
    Solve the given puzzle.

    If target is a point, return the steps needed to get to that point
    If fill is a number, return all points that can be reached in that many steps
    '''

    q = queue.Queue()
    visited = set()

    q.put((start, []))

    while not q.empty():
        (x, y), steps = q.get()
        if args.debug:
            print('{}, {} steps from origin, ~{} in queue'.format((x, y), len(steps), q.qsize()))
            if target:
                render(steps, target)
            else:
                render(visited)
            print()

        # If we're in target mode and found the target, return how we got there
        if target and x == target[0] and y == target[1]:
            return steps

        # If we're in fill mode and have gone too far, bail out
        if fill and len(steps) > fill:
            continue

        visited.add((x, y))

        # Add any neighbors we haven't seen yet (don't run into walls)
        for xd, yd in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            if (x + xd, y + yd) in visited:
                continue

            if wall(x + xd, y + yd):
                continue

            q.put(((x + xd, y + yd), steps + [(x, y)]))

    if fill:
        return visited

    raise Exception('Cannot reach target')

if args.target:
    target_x, target_y = map(int, args.target.split(','))
    solution = solve((1, 1), target = (target_x, target_y))
else:
    fill = int(args.fill)
    solution = solve((1, 1), fill = fill)

print('=== Solution ===')
render(solution)
print()
print('{} steps/points'.format(len(solution)))
