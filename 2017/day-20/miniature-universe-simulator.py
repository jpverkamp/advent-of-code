#!/usr/bin/env python3

import itertools

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--max-ticks', type = int, default = 1000, help = 'Number of ticks to run collision simulation')

origin = (0, 0, 0)

def parse_point(p):
    return tuple(map(int, p.split('=')[-1][1:-1].split(',')))

points = [
    tuple(map(parse_point, line.split(', ')))
    for line in lib.input()
]

def simulate(p, v, a):
    '''
    Yield points along a curve until they are moving away from zero after speeding up.

    If max_ticks is specified, yield that many points.
    If not, yield until the particle is accelerating away from the origin to infinity.
    '''

    last_speed = None
    speeding_up = False

    last_distance_to_zero = None
    moving_away = False

    for tick in itertools.count():
        yield p

        v = lib.vector_add(v, a)
        p = lib.vector_add(p, v)

# PART 1
# Calculate which point is acclerating away from the origin the slowest
# Assumptions:
# - All particles will eventually move away from the origin increasingly quickly
# - The particle accelerating the slowest is the one that will eventually fall behind
# - If two have equal acceleration, the one that started closer to the origin wins

slowest_acceleration = None
slowest_distance = None
slowest_index = None

for index, point in enumerate(points):
    lib.log(f'=== Checking {index} / {len(points)}: {point} ===')

    p, v, a = point
    acceleration = lib.vector_distance(a, origin)

    distance = lib.vector_distance(p, origin)

    new_best = (
        slowest_acceleration == None
        or acceleration < slowest_acceleration
        or (acceleration == slowest_acceleration and distance < slowest_distance)
    )

    if new_best:
        lib.log(f'New slowest acceleration point {index}: {acceleration}')

        slowest_acceleration = acceleration
        slowest_distance = distance
        slowest_index = index

print(f'{slowest_index} has the slowest acceleration ({slowest_acceleration})')

# PART 2
# Calculate how many points are left after collisions
# Assumptions:
# - Stop if all particles are moving apart (maximum pairwise distance is increasing)
#   (Technically you also have to know that all particles are currently accelerating)

simulators = [simulate(*point) for point in points]
last_max_distance = None

for tick in itertools.count():
    lib.log(f'=== Simulating collisions tick {tick}, ({len(simulators)} left) ===')

    current_points = [next(simulator) for simulator in simulators]

    # Remove any particles that have collided

    to_remove = {
        i
        for i, pa in enumerate(current_points)
        for j, pb in enumerate(current_points)
        if i != j and pa == pb
    }

    if to_remove:
        lib.log(f'{list(to_remove)} collided on tick {tick}')

    simulators = [
        simulator
        for index, simulator in enumerate(simulators)
        if index not in to_remove
    ]

    # Check if everything is moving apart (we can stop then)

    max_distance = max(
        lib.vector_distance(p1, p2)
        for p1 in current_points
        for p2 in current_points
    )

    lib.log(f'Maximum distance: {max_distance} (last: {last_max_distance})')

    if last_max_distance:
        if max_distance > last_max_distance:
            break
    else:
        last_max_distance = max_distance

print(f'{len(simulators)} left after collisions')
