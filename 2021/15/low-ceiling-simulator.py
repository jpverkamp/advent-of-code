import logging
import typer

from collections import *
from dataclasses import dataclass
from typing import *

app = typer.Typer()

Point = Tuple[int, int]
ORTHAGONAL = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def disableable_cache(x): return x


def load(file: TextIO) -> Mapping[Point, int]:
    return {
        (x, y): int(c)
        for y, line in enumerate(file)
        for x, c in enumerate(line.strip())
    }


def explore(map: Mapping[Point, int]) -> List[Point]:
    '''
    Version 1: Brute force, update all paths and iterate until stable. 
    '''

    width = max(x + 1 for (x, _) in map)
    height = max(y + 1 for (_, y) in map)

    bottom_right = (width - 1, height - 1)

    best_paths = {
        bottom_right: (1, [bottom_right])
    }

    # Pass until the best path map stops changing
    best_paths_changed = True
    generation = 0

    while best_paths_changed:
        best_paths_changed = False
        generation += 1
        logging.info(f'{generation=}, {len(best_paths)}/{len(map)} paths populated')

        for x in range(width):
            for y in range(height):
                neighbor_scores = [
                    best_paths[x + xd, y + yd]
                    for xd, yd in ORTHAGONAL
                    if (x + xd, y + yd) in best_paths
                ]

                if not neighbor_scores:
                    continue

                # Find the best neighbor
                best_score, best_path = min(neighbor_scores)

                new_score = map[x, y] + best_score
                new_path = [(x, y)] + best_path

                # If we haven't found a path for this point or we found a better one, update
                if not (x, y) in best_paths or new_score < best_paths[x, y][0]:
                    best_paths_changed = True
                    best_paths[x, y] = (new_score, new_path)

    return best_paths[0, 0][1]


def explore_2(map: Mapping[Point, int]) -> List[Point]:
    '''
    Version 2: Scan from the bottom right.

    NOTE: This version cannot handle paths that move up or left.
    '''

    width = max(x + 1 for (x, _) in map)
    height = max(y + 1 for (_, y) in map)

    bottom_right = (width - 1, height - 1)

    best_paths = {
        bottom_right: (1, [bottom_right])
    }

    # Start with the two points adjacent to bottom right
    to_scan = [
        (width - 2, height - 1),
        (width - 1, height - 2),
    ]

    while to_scan:
        (x, y) = to_scan[0]
        to_scan = to_scan[1:]

        # If it's out of bounds, skip
        if (x, y) not in map:
            continue

        # If it's already been scanned, skip
        if (x, y) in best_paths:
            continue

        logging.info(f'Scanning ({x}, {y}), {len(to_scan)}/{len(map)} remaining')

        # Find the best path to get to this point
        neighbors = [
            best_paths[x + xd, y + yd]
            for xd, yd in ORTHAGONAL
            if (x + xd, y + yd) in best_paths
        ]

        best_score, best_path = min(neighbors)

        best_paths[x, y] = (
            map[x, y] + best_score,
            [(x, y)] + best_path
        )

        # Add adjacent points to scan next
        # We'll handle duplicates and out of bounds at the top of the loop
        for xd, yd in ORTHAGONAL:
            to_scan.append((x + xd, y + yd))

    return best_paths[0, 0][1]


def explore_3(map: Mapping[Point, int]) -> List[Point]:
    '''
    Version 3: Iterate until stable again, but this time from the bottom right.
    '''

    width = max(x + 1 for (x, _) in map)
    height = max(y + 1 for (_, y) in map)

    bottom_right = (width - 1, height - 1)

    best_paths = {
        bottom_right: (1, [bottom_right])
    }

    changed_paths = 1
    while changed_paths:
        changed_paths = 0

        for x in range(width - 1, -1, -1):
            for y in range(height - 1, -1, -1):
                if (x, y) == bottom_right:
                    continue

                # Find the best path to get to this point
                neighbors = [
                    best_paths[x + xd, y + yd]
                    for xd, yd in ORTHAGONAL
                    if (x + xd, y + yd) in best_paths
                ]

                best_score, best_path = min(neighbors)
                new_score = map[x, y] + best_score
                new_path = [(x, y)] + best_path

                if (x, y) not in best_paths or new_score < best_paths[x, y][0]:
                    best_paths[x, y] = (new_score, new_path)
                    changed_paths += 1

        logging.info(f'Finished iteration, {changed_paths} paths changed')

    return best_paths[0, 0][1]


def explore_astar(map: Mapping[Point, int]) -> List[Point]:
    '''
    Solve the problem using the A* algorithm.
    '''

    from queue import PriorityQueue

    width = max(x + 1 for (x, _) in map)
    height = max(y + 1 for (_, y) in map)

    start = (0, 0)
    goal = (width - 1, height - 1)

    def h(p):
        return abs(goal[0] - p[0]) + abs(goal[1] - p[1])

    sources: MutableMapping[Point, Tuple[Optional[Point], int]] = {
        start: (None, 0)
    }

    q: PriorityQueue = PriorityQueue()

    q.put((0, start))

    while q:
        logging.info(f'Queue size: {q.qsize()}')

        _, current = q.get()
        (x, y) = current

        if current == goal:
            break

        for xd, yd in ORTHAGONAL:
            next = (x + xd, y + yd)
            if next not in map:
                continue

            new_cost = sources[current][1] + map[current]

            if next not in sources or new_cost < sources[next][1]:
                sources[next] = (current, new_cost)
                q.put((new_cost + h(next), next))

    logging.info(f'Found solution after evaluating {len(sources)} paths')

    best_path = []
    current = goal

    while current:
        best_path.append(current)
        current, score = sources[current]

    best_path = list(reversed(best_path))

    return best_path


@app.command()
def part1(file: typer.FileText):

    map = load(file)
    best_path = explore(map)
    best_score = sum(map[x, y] for (x, y) in best_path[1:])

    logging.info(f'{best_path=}')
    print(f'{best_score=}')


@app.command()
def part2(file: typer.FileText):

    original_map = load(file)
    map: MutableMapping[Point, int] = dict(original_map)

    width = max(x + 1 for (x, _) in map)
    height = max(y + 1 for (_, y) in map)

    for bigx in range(5):
        for bigy in range(5):
            offset = bigy + bigy

            for x in range(width):
                for y in range(height):
                    newx = x + bigx * width
                    newy = y + bigy * height

                    map[newx, newy] = (map[x, y] + bigx + bigy)
                    if map[newx, newy] > 9:
                        map[newx, newy] -= 9

    best_path = explore(map)
    best_score = sum(map[x, y] for (x, y) in best_path[1:])

    logging.info(f'{best_path=}')
    print(f'{best_score=}')


@app.callback()
def enableFlags(cache: bool = False, debug: bool = False, version: int = 1):
    if debug:
        import coloredlogs  # type: ignore
        coloredlogs.install(level=logging.INFO)

    if cache:
        import functools
        global disableable_cache
        disableable_cache = functools.cache

    global explore, explore_2, explore_3, explore_astar

    if version == 1:
        logging.info('Running version 1')
    elif version == 2:
        logging.info('Running version 2')
        explore = explore_2
    elif version == 3:
        logging.info('Running version 3')
        explore = explore_3
    elif version == 4:
        logging.info('Running version 4 (A*)')
        explore = explore_astar
    else:
        logging.critical(f'Unknown version {version}')
        exit(version)


if __name__ == '__main__':
    app()
