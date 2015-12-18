#!/usr/bin/env python3

import copy
import sys

class Grid(object):
    def __init__(self, str):
        self.data = [
            [(char == '#') for char in line]
            for line in str.split()
        ]
        self.width = len(self.data)
        self.height = len(self.data[0])

    def __getitem__(self, pt):
        row, col = pt
        if 0 <= row < self.height and 0 <= col < self.width:
            return self.data[row][col]
        else:
            return False

    def neighbors(self, row, col):
        return sum(
            (1 if self[row + row_delta, col + col_delta] else 0)
            for row_delta in range(-1, 2)
            for col_delta in range(-1, 2)
        ) - (1 if self[row, col] else 0)

    def step(self):
        new_data = copy.deepcopy(self.data)

        for row in range(self.height):
            for col in range(self.width):
                if self[row, col]:
                    new_data[row][col] = (2 <= self.neighbors(row, col) <= 3)
                else:
                    new_data[row][col] = (self.neighbors(row, col) == 3)

        self.data = new_data

    def __repr__(self):
        return '\n'.join(
            ''.join(
                '#' if self[row, col] else '.'
                for col in range(self.width)
            )
            for row in range(self.height)
        )

    def __len__(self):
        return sum(
            1 if self[row, col] else 0
            for row in range(self.height)
            for col in range(self.width)
        )

if __name__ == '__main__':
    grid = Grid(sys.stdin.read())

    for i in range(int(sys.argv[1])):
        grid.step()

    print(grid)
    print(len(grid))
