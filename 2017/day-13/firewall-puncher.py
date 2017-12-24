#!/usr/bin/env python3

import itertools

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--tick', type = int, help = 'Calculate the severity of running the firewall at the given tick', group = 'mode')
lib.add_argument('--safest', action = 'store_true', help = 'Find the first tick that is completely safe', group = 'mode')

firewalls = {
    int(depth): int(range)
    for line in lib.input()
    for depth, range in [line.split(': ')]
}

def calculate_severity(delay, return_pass_all = False):
    '''
    Calulate how severe the alarm is if you start at the given tick.

    If return_pass_all is set, return True if you hit no walls and False if you
    hit any (even if the severity would be 0).
    '''

    total_severity = 0

    for depth in firewalls:
        lib.log(f'Checking firewall {depth}')

        range = firewalls[depth]

        cycle_length = (range - 1) * 2
        position = (delay + depth) % cycle_length

        if position > range:
            position = 2 * range - position

        lib.log(f'Firewall will be at {position}')

        if position == 0:
            severity = depth * firewalls[depth]
            total_severity += severity
            lib.log(f'DETECTED! Added {severity} to severity')

            if return_pass_all:
                return False

    if return_pass_all:
        return True
    else:
        return total_severity

if lib.param('tick') != None:
    tick = lib.param('tick')
    severity = calculate_severity(tick)
    print(f'{tick}: severity {severity}')

elif lib.param('safest'):
    safe_tick = None
    for tick in itertools.count():
        lib.log('=== {} ===', tick)
        if calculate_severity(tick, return_pass_all = True):
            safe_tick = tick
            break

    print(f'safe: {safe_tick}')
