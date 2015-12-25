#!/usr/bin/env python3

import sys

target_row = int(sys.argv[1])
target_col = int(sys.argv[2])

row = 1
col = 1
val = 20151125

while True:
    if target_row == row and target_col == col:
        print(val)
        sys.exit(0)
    else:
        val = (val * 252533) % 33554393

    if row == 1:
        row = col + 1
        col = 1
    else:
        row -= 1
        col += 1
