#!/usr/bin/env python3

import imp
import lib
import pprint
import sys

part1 = imp.load_source('part1', 'part-1-priority-queue.py')

if __name__ == '__main__':
    boss = lib.Entity()
    for line in sys.stdin:
        key, val = line.strip().split(': ')
        boss[key] = int(val)

    player = lib.Entity(**{
        'Hit Points': 50,
        'Mana Points': 500,
        'Active Spells': [lib.HardMode()],
        'History': [],
    })

    best_player = part1.solve_mode(player, boss)

    pprint.pprint(best_player)
