#!/usr/bin/env python3

import imp
import lib
import pprint
import sys

part1 = imp.load_source('part1', 'part-1-monte-carlo.py')

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

    simulations, wins, best_player = part1.monte_carlo(player, boss)

    print('{} simulations run, player won {}'.format(simulations, wins))
    pprint.pprint(best_player)
