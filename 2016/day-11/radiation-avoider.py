#!/usr/bin/env python3

import argparse
import copy
import itertools
import re
import time

parser = argparse.ArgumentParser()
parser.add_argument('input')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

class Thing(object):
    def __init__(self, element): self.element = element
    def __repr__(self): return '{}<{}>'.format(self.__class__.__name__, self.element)
    def __hash__(self): return hash(repr(self))
    def __eq__(self, other): return repr(self) == repr(other)
    def __lt__(self, other): return repr(self) < repr(other)

class Generator(Thing): pass
class Microchip(Thing): pass

class State(object):
    def __init__(self, floors, elevator = 0, steps = None):
        self._elevator = elevator
        self._floors = floors
        self._steps = steps or []
        self._hash = hash(repr(self))

    def __repr__(self):
        '''Simple output for repr that doesn't include steps (since this is used by hash).'''

        # Optimization: Parts are interchangeable, rewrite them by order
        # This will assign an index the first time it sees a name and use that any more
        # So parts will always be ordered from lowest to highest, ties broken alphabetically
        def ordered_rewrite(m, cache = {}):
            type, name = m.groups()

            if name not in cache:
                cache[name] = str(len(cache))

            return '{}{}'.format(type[0], cache[name])

        floor_strings = []
        for items in self._floors:
            if items:
                floor_strings.append(' '.join(repr(item) for item in sorted(items)))
            else:
                floor_strings.append('∅')

        return re.sub(
            r'(Microchip|Generator)<([^<>]+)>',
            ordered_rewrite,
            'State<{}, {}>'.format(self._elevator, ', '.join(floor_strings)),
        )

    def __str__(self):
        '''Nicer output for str(state) that includes steps.'''

        floors = []
        for i, floor in enumerate(self._floors):
            level_part = ('[{}]' if self._elevator == i else ' {} ').format(i + 1)
            item_part = ' '.join(str(item) for item in sorted(floor))
            floors.append(level_part + ' ' + item_part)
        return '\n'.join(reversed(floors))

    def __hash__(self):
        return self._hash

    def __eq__(self, other):
        return hash(self) == hash(other)

    def steps(self):
        '''Return the steps taken by this object.'''

        return copy.copy(self._steps)

    def move(self, delta, items):
        '''
        Return a new state resulting from moving the elevator by delta (-1/+1) and take items with it.

        If the new state isn't valid, return None.
        '''

        if not (0 <= self._elevator + delta < len(self._floors)):
            return

        new_floors = copy.deepcopy(self._floors)
        for item in items:
            new_floors[self._elevator].remove(item)
            new_floors[self._elevator + delta].add(item)

        new_state = State(
            floors = new_floors,
            elevator = self._elevator + delta,
            steps = self._steps + [(delta, items)],
        )

        if new_state.is_valid():
            return new_state

    def is_valid(self):
        '''
        A state is invalid if a microchip is on a floor with a generator, but
        does not have its own generator.
        '''

        for floor in self._floors:
            chips = {item for item in floor if isinstance(item, Microchip)}
            generators = {item for item in floor if isinstance(item, Generator)}

            # If there are no generators, the chips are safe
            if not generators:
                continue

            # At least one powered chip
            # If there are any chips that don't have a matching generator, they are not safe
            if any(chip for chip in chips if not Generator(chip.element) in generators):
                return False

        return True

    def is_solved(self):
        '''If all items are on the top floor, the puzzle is solved.'''

        return not any(self._floors[:-1])

    def next_states(self):
        '''Return all valid next states possible from moving 1-2 items up or down a floor.'''

        for delta in [-1, 1]:
            for count in [1, 2]:
                for items in itertools.combinations(self._floors[self._elevator], count):
                    new_state = self.move(delta, items)
                    if new_state:
                        yield new_state

# Read the initial state from the given input file
re_object = re.compile(r'(\w+)(?:-compatible)? (generator|microchip)')
with open(args.input, 'r') as fin:
    floors = []
    for line in fin:
        if not line.strip() or line.startswith('#'):
            continue

        floor = set()
        for element, type in re_object.findall(line):
            if type == 'generator':
                floor.add(Generator(element))
            elif type == 'microchip':
                floor.add(Microchip(element))

        floors.append(floor)

    initial_state = State(floors)

# Use a BFS to find the fastest solution
def solve(initial_state):
    start = time.time()
    count = 0
    queue = [initial_state]
    duplicates = {initial_state}
    duplicate_count = 0

    while queue:
        state = queue.pop(0)
        count += 1

        if args.debug:
            print('===== ===== ===== ===== =====')
            print(' Runtime: {:.2f} seconds'.format(time.time() - start))
            print(' Checked: {}'.format(count))
            print('    Rate: {:.2f} / second'.format(count / (time.time() - start)))
            print('In queue: {}'.format(len(queue)))
            print(' Skipped: {}'.format(duplicate_count))
            print('   Steps: {}'.format(len(state.steps())))
            print()
            print(repr(state))
            print()
            print(state)
            print('===== ===== ===== ===== =====')
            #input('Press any key...')

        if state.is_solved():
            return state

        for next_state in state.next_states():
            if next_state in duplicates:
                duplicate_count += 1
            else:
                queue.append(next_state)
                duplicates.add(next_state)
final_state = solve(initial_state)

print('''\
=== Solution ===

Initial state:
{initial}

Final state:
{final}

Steps:
{steps}
'''.format(
    count = len(final_state.steps()),
    initial = initial_state,
    final = final_state,
    steps = '\n'.join(
        '{}: {} {}'.format(i + 1, delta, items)
        for i, (delta, items) in enumerate(final_state.steps())
    ),
))
