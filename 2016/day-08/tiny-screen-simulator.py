#!/usr/bin/env python3

import argparse
import regex as re

parser = argparse.ArgumentParser()
parser.add_argument('--width', type = int, default = 50)
parser.add_argument('--height', type = int, default = 6)
parser.add_argument('input')
args = parser.parse_args()

re_rect = re.compile(r'rect (?P<width>\d+)x(?P<height>\d+)')
re_rotate = re.compile(r'rotate (?P<mode>\w+) (?P<xy>[xy])=(?P<index>\d+) by (?P<offset>\d+)')

class Screen(object):
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.data = [([False] * self.width) for i in range(self.height)]

    def __str__(self):
        return 'Screen<{}, {}>\n{}'.format(
            self.width,
            self.height,
            '\n'.join(''.join(('#' if el else '-') for el in row) for row in self.data),
        )

    def __len__(self):
        return sum(
            (1 if self[x, y] else 0)
            for x in range(self.width)
            for y in range(self.height)
        )

    def __getitem__(self, pt):
        (x, y) = pt
        return self.data[y % self.height][x % self.width]

    def __setitem__(self, pt, value):
        (x, y) = pt
        self.data[y % self.height][x % self.width] = value

screen = Screen(args.width, args.height)

with open(args.input, 'r') as fin:
    for line in fin:
        print(line)

        m_rect = re_rect.match(line)
        m_rotate = re_rotate.match(line)

        if m_rect:
            for x in range(int(m_rect.group('width'))):
                for y in range(int(m_rect.group('height'))):
                    screen[x, y] = True

        elif m_rotate:
            offset = int(m_rotate.group('offset'))
            index = int(m_rotate.group('index'))

            if m_rotate.group('mode') == 'column':
                new_data = [screen[index, y + yd] for yd in range(screen.height)]
                for yd in range(screen.height):
                    screen[index, y + yd + offset] = new_data[yd]
            else:
                new_data = [screen[x + xd, index] for xd in range(screen.width)]
                for xd in range(screen.width):
                    screen[x + xd + offset, index] = new_data[xd]

        print(screen)
        print()

print('active cells', len(screen))
