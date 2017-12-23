#!/usr/bin/env python3

import argparse
import copy
import fileinput
import functools
import logging
import queue
import re
import tabulate

parser = argparse.ArgumentParser()
parser.add_argument('files', nargs = '*', help = 'List of filters to apply')
parser.add_argument('--guess', action = 'store_true', help = 'Make a best guess rather than exhaustively solving')
parser.add_argument('--debug', action = 'store_true')
parser.add_argument('--print_usage', action = 'store_true')
args = parser.parse_args()

if args.debug:
    logging.basicConfig(level = logging.INFO)

sizes = {}
usage = {}

for line in fileinput.input(args.files):
    if not line.startswith('/dev/grid/'):
        continue

    name, size, used, available, percent = line.strip().split()
    _, xs, ys = name.split('/')[-1].split('-')
    x = int(xs[1:])
    y = int(ys[1:])

    sizes[x, y] = int(size[:-1])
    usage[x, y] = int(used[:-1])

# Part 1 - determine how many viable pairs there are
viable_pairs = set()

for a in sizes:
    if usage[a] == 0:
        continue

    for b in sizes:
        if a == b:
            continue

        if usage[a] + usage[b] <= sizes[b]:
            viable_pairs.add((a, b))

print('{} viable pairs'.format(len(viable_pairs)))

# Part 2 - try to move data from top right to top left
max_x = max(x for (x, y) in sizes)
max_y = max(y for (x, y) in sizes)

# State is (current usage, (goal x, goal y), step count)
# Target state is (*, (0, 0), *)

def hash_state(usage, goal):
    return hash('{} @ {}'.format(
        goal,
        [usage[x, y] for x in range(max_x + 1) for y in range(max_y + 1)]
    ))

def print_usage(usage, goal):
    print(tabulate.tabulate([
        [
            ('[{}/{}]' if (x, y) == goal else '{}/{}').format(usage[x, y], sizes[x, y])
            for x in range(max_x + 1)
        ]
        for y in range(max_y + 1)
    ]))
    print()

def print_usage_icons(usage, goal):
    '''Print based on the hint in the puzzle.'''

    for y in range(max_y + 1):
        for x in range(max_x + 1):
            if (x, y) == goal:
                output = 'G'
            elif usage[x, y] == 0:
                output = '@'
            elif sizes[x, y] > 500:
                output = '#'
            else:
                output = '.'

            print(output, end = '')
        print()

if args.guess:
    logging.info('Trying to make a best guess')
    if args.debug:
        print_usage_icons(usage, (max_x, 0))

    # Find the empty node
    for x in range(max_x + 1):
        for y in range(max_y + 1):
            if usage[x, y] == 0:
                (empty_x, empty_y) = empty = (x, y)

    # Find walls (nodes with more than 500T data)
    walls = {
        (x, y)
        for x in range(max_x + 1)
        for y in range(max_y + 1)
        if sizes[x, y] > 500
    }

    # Use dynamic programming to find the minimum distance between two points
    logging.info('Creating distance map')
    distance_from_empty = {}
    to_calculate = queue.Queue()

    to_calculate.put((empty, 0))
    while not to_calculate.empty():
        point, distance = to_calculate.get()
        if point in distance_from_empty:
            continue

        distance_from_empty[point] = distance

        (x, y) = point
        for xd, yd in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            neighbor = (x + xd, y + yd)
            if 0 <= x + xd <= max_x and 0 <= y + yd <= max_y and neighbor not in walls:
                to_calculate.put((neighbor, distance + 1))

    # Move to immediately beside the goal
    distance_to_goal = distance_from_empty[(max_x - 1, 0)]

    # Now it takes 5 to move the goal one left and reset (except the last time)
    distance_to_zero = 5 * (max_x - 1) + 1

    print('Best guess = {} ({} to goal + {} to zero)'.format(
        distance_to_goal + distance_to_zero,
        distance_to_goal,
        distance_to_zero,
    ))



else:
    logging.info('Finding the real solution')

    initial_state = (copy.deepcopy(usage), (max_x, 0), 0)

    q = queue.Queue()
    q.put(initial_state)
    seen = set()

    while True:
        if q.empty():
            raise Exception('Ran out of possibilities to test')

        current_usage, goal, steps = q.get()

        current_hash = hash_state(current_usage, goal)
        if current_hash in seen:
            continue
        else:
            seen.add(current_hash)

        logging.info('Testing a solution with {} steps, goal data at {}, ~{} in queue'.format(steps, goal, q.qsize()))
        if args.debug and not args.print_usage:
            print_usage_icons(current_usage, goal)
        if args.print_usage:
            print_usage(current_usage, goal)

        if (0, 0) == goal:
            print('Found a solution in {} steps'.format(steps))
            break

        # Try moving everything everywhere
        # TODO: Improve this :)
        nodes_added = 0

        for x in range(max_x + 1):
            for y in range(max_y + 1):
                for xd, yd in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                    if not (0 <= x + xd <= max_x and 0 <= y + yd <= max_y):
                        continue

                    # Make sure we can move the given data
                    total_size = current_usage[x, y] + current_usage[x + xd, y + yd]
                    if total_size <= sizes[x + xd, y + yd]:
                        new_usage = copy.deepcopy(current_usage)
                        new_usage[x, y] = 0
                        new_usage[x + xd, y + yd] = total_size

                        if (x, y) == goal:
                            q.put((new_usage, (x + xd, y + yd), steps + 1))
                        else:
                            q.put((new_usage, goal, steps + 1))

                        nodes_added += 1

        logging.info('{} nodes added'.format(nodes_added))
