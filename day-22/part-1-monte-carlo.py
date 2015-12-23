#!?usr/bin/env python3

import copy
import lib
import pprint
import random
import sys
import time

try:
    TIME_TO_RUN = int(sys.argv[1])
except:
    TIME_TO_RUN = 60

def random_spells():
    while True:
        yield random.choice(lib.spells)

class GameOverException(Exception):
    def __init__(self, player_won, reason):
        self.player_won = player_won
        self.reason = reason

def check_game_over(player, boss):
    if boss['Hit Points'] <= 0:
        raise GameOverException(True, 'boss died')

    if player['Hit Points'] <= 0:
        raise GameOverException(False, 'player died')

def fight(player, boss, spell_iterator):
    while True:
        check_game_over(player, boss)

        # --- Player turn ---

        player['History'].append('<< Player turn: HP={}, MP={}, MS={}, BossHP={} >>'.format(
            player['Hit Points'],
            player['Mana Points'],
            player['Mana Spent'],
            boss['Hit Points'],
        ))
        player.tick_active_spells(boss)
        check_game_over(player, boss)

        for i, potential_spell in enumerate(spell_iterator):
            if i >= 10:
                raise GameOverException(False, 'failed to cast 10 spells')

            if potential_spell.Cost > player['Mana Points']:
                continue

            spell = potential_spell()
            if spell in player['Active Spells']:
                continue

            player['History'].append('Player casts {}'.format(potential_spell.__name__))
            player['Mana Points'] -= potential_spell.Cost
            player['Mana Spent'] += potential_spell.Cost

            spell.cast(player, boss)
            check_game_over(player, boss)

            if spell.Duration:
                player['Active Spells'].append(spell)

            break

        # --- Boss turn ---

        player['History'].append('<< Boss turn: HP={}, MP={}, MS={}, BossHP={} >>'.format(
            player['Hit Points'],
            player['Mana Points'],
            player['Mana Spent'],
            boss['Hit Points'],
        ))

        player.tick_active_spells(boss)
        check_game_over(player, boss)

        player.damage(boss['Damage'])
        check_game_over(player, boss)

if __name__ == '__main__':
    boss = lib.Entity()
    for line in sys.stdin:
        key, val = line.strip().split(': ')
        boss[key] = int(val)

    player = lib.Entity(**{
        'Hit Points': 50,
        'Mana Points': 500,
        'Active Spells': [],
        'History': [],
    })

    start = time.time()
    best_player = {'Mana Spent': float('inf')}
    simulations = 0
    wins = 0

    while True:
        if time.time() - start > TIME_TO_RUN:
            break

        simulations += 1
        current_boss = copy.deepcopy(boss)
        current_player = copy.deepcopy(player)

        try:
            fight(current_player, current_boss, random_spells())
        except GameOverException as game_over:
            if game_over.player_won:
                wins += 1
                if current_player['Mana Spent'] < best_player['Mana Spent']:
                    print('New best:', current_player['Mana Spent'])
                    best_player = current_player

print('{} simulations run, player won {}'.format(simulations, wins))
pprint.pprint(best_player)
