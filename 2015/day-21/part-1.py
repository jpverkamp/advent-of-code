#!/usr/bin/env python3

import lib

best_player = {'Cost': float("inf")}
for player in lib.all_players():
    if lib.player_wins(player, lib.get_enemy()):
        if player['Cost'] < best_player['Cost']:
            best_player = player

print(best_player['Cost'])
