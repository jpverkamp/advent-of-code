import itertools
import logging
import re

def math(expression, variables):
    '''Safely evaluate a mathematical expression with the given variables.'''

    if re.match(r'[^0-9a-z+\-*/ ]', expression):
        raise Exception('Unsafe expression: {}'.format(expression))

    # TODO: Make this actually safe.

    return eval(expression, globals(), variables)

class SpiralGrid():
    '''
    Generate a spiral grid that looks like this:
    17  16  15  14  13
    18   5   4   3  12
    19   6   1   2  11
    20   7   8   9  10
    21  22  23---> ...

    The point (0, 0) is 1. x runs left to right, y from top to bottom. So the
    point 12 is at (2, -1).
    '''

    def __init__(self):
        self._indexes = {}
        self._points = {}

        def make_spiral():
            index = 1
            (x, y) = (0, 0)

            yield index, (x, y)

            # Build the layers outwards
            for layer in itertools.count(1):
                # Each layer starts by going right and down one (we'll go back up before yielding)
                x += 1
                y += 1

                # Go through the four sides, up then left then down then right
                # Repeat 2*layer times per side
                for xd, yd in [(0, -1), (-1, 0), (0, 1), (1, 0)]:
                    for step in range(2 * layer):
                        index += 1
                        x += xd
                        y += yd
                        yield index, (x, y)

        self._generator = make_spiral()

    def __getitem__(self, key):
        '''
        Given an index or point return the other.

        If we're given an integer, it's an index, return the point.
        If we're given a tuple, it's a point, return the index.

        Either way, generate as much data as we need and don't have.
        '''

        if isinstance(key, int):
            field = self._indexes
        elif isinstance(key, str):
            key = int(key)
            field = self._indexes
        elif isinstance(key, tuple) and len(key) == 2:
            field = self._points
        else:
            raise ValueError

        while key not in field:
            index, point = next(self._generator)
            self._indexes[index] = point
            self._points[point] = index
            logging.info('Generated new point in spiral grid: {} = {}'.format(index, point))

        return field[key]
