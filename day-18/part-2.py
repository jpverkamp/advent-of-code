#!/usr/bin/env python3

import imp
import sys

part1 = imp.load_source('part1', 'part-1.py')

class FixedCornerGrid(part1.Grid):
    def __getitem__(self, pt):
        row, col = pt
        if (row in (0, self.width - 1) and col in (0, self.height - 1)):
            return True
        else:
            return part1.Grid.__getitem__(self, pt)

if __name__ == '__main__':
    grid = FixedCornerGrid(sys.stdin.read())

    for i in range(int(sys.argv[1])):
        grid.step()

    print(grid)
    print(len(grid))
