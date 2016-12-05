import copy
import sys

shop = {}
category = None

with open('shop.txt', 'r') as fin:
    for line in fin:
        line = line.strip()
        if not line:
            continue

        if ':' in line:
            category = line.split(':')[0]
            shop[category] = []
            continue

        name, cost, damage, armor = line.rsplit(maxsplit = 3)

        shop[category].append({
            'Name': name,
            'Cost': int(cost),
            'Damage': int(damage),
            'Armor': int(armor),
        })

# Allow for no armor or rings
shop['Armor'].append({'Name': None, 'Cost': 0, 'Damage': 0, 'Armor': 0})
shop['Rings'].append({'Name': None, 'Cost': 0, 'Damage': 0, 'Armor': 0})

enemy = {}
for line in sys.stdin:
    key, val = line.strip().split(':')
    enemy[key] = int(val)

def get_enemy():
    return copy.copy(enemy)

def player_wins(player, enemy):
    while True:
        enemy['Hit Points'] -= max(1, player['Damage'] - enemy['Armor'])
        if enemy['Hit Points'] <= 0:
            return True

        player['Hit Points'] -= max(1, enemy['Damage'] - player['Armor'])
        if player['Hit Points'] <= 0:
            return False

def all_players():
    for weapon in shop['Weapons']:
        for armor in shop['Armor']:
            for left_ring in shop['Rings']:
                for right_ring in shop['Rings']:
                    # Cannot have two of the same ring unless they're both None
                    if left_ring and right_ring and left_ring == right_ring:
                        continue

                    items = [weapon, armor, left_ring, right_ring]

                    player = {
                        'Hit Points': 100,
                        'Items': [item['Name'] for item in items if item['Name']],
                        'Damage': sum(item['Damage'] for item in items),
                        'Armor': sum(item['Armor'] for item in items),
                        'Cost': sum(item['Cost'] for item in items),
                    }

                    yield player
