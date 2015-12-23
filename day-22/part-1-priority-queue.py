#!/usr/bin/env python3

import collections
import copy
import lib
import pprint
import queue
import sys
import time

DEBUG_PRINT_FREQUENCY = 5.0

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

queue_breaker = 0

states = queue.PriorityQueue()
states.put((0, queue_breaker, player, boss))

# This will be used to break ties in the queue since Entities are not orderable
queue_breaker += 1
start = time.time()
last_print = start

while not states.empty():
    mana_spent, _, player, boss = states.get()

    now = time.time()
    if now - last_print > DEBUG_PRINT_FREQUENCY:
        last_print = now
        print('{} in queue, {} done, {} mana spent, {:.0f} seconds elapsed'.format(
            states.qsize(),
            queue_breaker,
            mana_spent,
            now - start,
        ))

    # If we win, because of the priority queue, this is the best solution
    if boss['Hit Points'] <= 0:
        print(player)
        sys.exit(0)

    # Player died, no point in continuing on this track
    if player['Hit Points'] <= 0:
        continue

    # --- Player's turn ---
    player = copy.deepcopy(player)
    boss = copy.deepcopy(boss)
    player['History'].append('>> Player Turn <<')
    player.tick_active_spells(boss)

    # Branch (see the copy below) to applying each possible spell for the player's turn
    for potential_spell in lib.spells:
        if player['Mana Points'] < potential_spell.Cost:
            continue

        spell = potential_spell()
        if spell in player['Active Spells']:
            continue

        current_player = copy.deepcopy(player)
        current_boss = copy.deepcopy(boss)

        # Cast the player's new spell
        current_player['Mana Points'] -= potential_spell.Cost
        spell.cast(current_player, current_boss)

        if spell['Duration']:
            current_player['Active Spells'].append(spell)

        current_player['History'].append(str(spell))

        # --- Boss's turn ---
        current_player['History'].append('>> Boss Turn <<')
        current_player.tick_active_spells(current_boss)
        current_player.damage(current_boss['Damage'])

        # Store the altered copies back in the queue
        states.put((mana_spent + spell.Cost, queue_breaker, current_player, current_boss))
        queue_breaker += 1
