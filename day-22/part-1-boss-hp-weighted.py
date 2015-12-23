#!/usr/bin/env python3

import copy
import lib
import pprint
import queue
import sys

try:
    BOSS_HP_WEIGHT = int(sys.argv[1])
except:
    BOSS_HP_WEIGHT = 10

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
states.put((0, 0, queue_breaker, player, boss))

# This will be used to break ties in the queue since Entities are not orderable
queue_breaker += 1

# <DEBUG>
import threading
import time

def print_updates():
    start = time.time()
    while True:
        time.sleep(5.0)

        print('{} in queue, {} processed after {:.0f} seconds'.format(
            states.qsize(),
            queue_breaker,
            time.time() - start,
        ))

t = threading.Thread(target = print_updates)
t.daemon = True
t.start()

best_mana_spent = float("inf")
best_player = None

while not states.empty():
    score, mana_spent, _, player, boss = states.get()

    # We've already found a better solution, skip this one
    if best_mana_spent < mana_spent:
        continue

    # If we win, because of the priority queue, this is the best solution
    if boss['Hit Points'] <= 0:
        if mana_spent < best_mana_spent:
            print('New best:', mana_spent)
            best_mana_spent = mana_spent
            best_player = player
            continue

    # Player died, no point in continuing on this track
    if player['Hit Points'] <= 0:
        continue

    # --- Player's turn ---
    player = copy.deepcopy(player)
    boss = copy.deepcopy(boss)
    #player['History'].append('>> Player Turn <<')
    player.tick_active_spells(boss)

    # Branch (see the copy below) to applying each possible spell for the player's turn
    for spell in lib.spells:
        if spell() in player['Active Spells']:
            continue

        if player['Mana Points'] < spell.Cost:
            continue

        current_player = copy.deepcopy(player)
        current_boss = copy.deepcopy(boss)

        # Cast the player's new spell
        current_player['Mana Points'] -= spell.Cost

        new_spell = spell()
        new_spell.cast(current_player, current_boss)

        if new_spell['Duration']:
            current_player['Active Spells'].append(new_spell)

        current_player['History'].append(spell.__name__)

        # --- Boss's turn ---
        #current_player['History'].append('>> Boss Turn <<')
        current_player.tick_active_spells(current_boss)
        current_player.damage(current_boss['Damage'])

        # Store the altered copies back in the queue
        new_score = mana_spent + spell.Cost + current_boss['Hit Points'] * BOSS_HP_WEIGHT
        states.put((new_score, mana_spent + spell.Cost, queue_breaker, current_player, current_boss))
        queue_breaker += 1

print('--- Player ---')
print(best_player)
print('--- Mana spent ---')
print(best_mana_spent)
