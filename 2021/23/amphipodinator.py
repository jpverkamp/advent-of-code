import logging
import typer
import queue

from collections import *
from dataclasses import dataclass
from typing import *
from frozendict import frozendict

app = typer.Typer()


def disableable_cache(x): return x


NEIGHBORS = [(-1, 0), (1, 0), (0, -1), (0, 1)]

ENERGY_COSTS = {
    'A': 1,
    'B': 10,
    'C': 100,
    'D': 1000,
}

TARGET_ROOMS = {
    'A': 3,
    'B': 5,
    'C': 7,
    'D': 9,
}


@dataclass(frozen=True)
class Point:
    x: int
    y: int

    def __repr__(self):
        return f'<{self.x}, {self.y}>'

    def distance(self, other: 'Point'):
        return abs(other.x - self.x) + abs(other.y - self.y)


@dataclass(frozen=True)
class State:
    width: int
    height: int
    walls: FrozenSet[Point]
    amphipods: Mapping[str, FrozenSet[Point]]

    @staticmethod
    def read(file: TextIO):

        width = 0
        height = 0
        walls = set()
        amphipods = {}

        for y, line in enumerate(file):
            height = max(y + 1, height)

            for x, c in enumerate(line.rstrip('\n')):
                width = max(x + 1, width)

                p = Point(x, y)

                if c == '#':
                    walls.add(p)

                elif c.isalpha():
                    amphipods.setdefault(c, set())
                    amphipods[c].add(p)

        return State(
            width,
            height,
            frozenset(walls),
            frozendict({
                c: frozenset(points)
                for c, points in amphipods.items()
            })
        )

    def __getitem__(self, p: Union[Tuple[int, int], Point]):
        if isinstance(p, tuple):
            x, y = p
            p = Point(x, y)

        if p in self.walls:
            return '#'

        for c, points in self.amphipods.items():
            if p in points:
                return c

        return ' '

    def __str__(self):
        map_data = []

        for y in range(self.height):
            for x in range(self.width):
                map_data.append(self[x, y])
            map_data.append('\n')

        return f'State<{self.width}x{self.height}>{{\n{"".join(map_data)}}}'

    def __lt__(self, other: 'State'):
        return False

    def floodfill(self, origin: Point) -> Generator[Point, None, None]:
        '''Generate all points reachable from a given location.'''

        scanned = set()
        to_scan = queue.Queue()
        to_scan.put(origin)

        while not to_scan.empty():
            p = to_scan.get_nowait()

            if p in scanned:
                continue

            scanned.add(p)

            if p != origin:
                yield p

            for xd, yd in NEIGHBORS:
                new_p = Point(p.x + xd, p.y + yd)

                if self[new_p] == ' ':
                    to_scan.put(new_p)

    def move(self, src: Point, dst: Point) -> 'State':
        '''Return a new state with the given amphipod moved from src to dst'''

        for delta_c, points in self.amphipods.items():
            if src in points:
                break

        return State(
            self.width,
            self.height,
            self.walls,
            frozendict({
                c: points if c != delta_c else points - {src} | {dst}
                for c, points in self.amphipods.items()
            })
        )

    def moves(self) -> Generator[Tuple[int, 'State'], None, None]:
        '''Generate the possible next states that we can get into from here.'''

        logging.debug('Generating moves')

        for c, points in self.amphipods.items():
            logging.debug(f'- Generating moves for {c=}')

            for p in points:
                logging.debug(f'-- Generating moves for {c=} @ {p=}')

                # Move amphipods from rooms to the hallway
                # Blocked amphipods won't floodfill, so this is fine
                if p.y >= 2:
                    logging.debug('-- Dealing with a room amphipod')
                    for new_p in self.floodfill(p):
                        # An amphipod will always stop in the hallway
                        if new_p.y != 1:
                            continue

                        # An amphipod will not stop immediately outside of any room
                        if new_p.x in TARGET_ROOMS.values():
                            continue

                        # Otherwise, yield this as a possibility
                        yield (p.distance(new_p) * ENERGY_COSTS[c], self.move(p, new_p))

                # An amphipod in the hallway will only go into a room
                # + only it's correct room
                # + only if there are no non-similar amphipods beneath it
                elif p.y == 1:
                    logging.debug('-- Dealing with a hallway amphipod')
                    for new_p in self.floodfill(p):
                        # An amphipod will not stay in the hallway once they are in it
                        if new_p.y == 1:
                            continue

                        # An amphipod will only got into it's own room
                        if new_p.x != TARGET_ROOMS[c]:
                            continue

                        # Blocking a room that we'd want to go deeper into
                        #if new_p.y >= 2 and any(self[new_p.x, y] != c for y in range(new_p.y + 1, self.height - 1)):
                        #    continue

                        # An amphipod will not enter a room that has other kinds of amphipods in it
                        if new_p.y >= 2 and any(
                                self[new_p.x, y] not in (c, ' ')
                                for y in range(2, self.height - 1)
                        ):
                            continue

                        # Otherwise, yield this as a possibility
                        yield (p.distance(new_p) * ENERGY_COSTS[c], self.move(p, new_p))

                # This shouldn't happen...
                else:
                    logging.critical(f'Confused amphipod: {c} @ {p}')

    def heuristic(self) -> int:
        '''Guess how many movies it will take to get to the final state.'''

        # For any amphis not in the target room, calculate a guess cost to get there
        # This will go to the hallway and into their room ignoring everything in their way
        return sum(
            (
                p.y - 1                       # To the hallway, 0 if in hallway already
                + abs(p.x - TARGET_ROOMS[c])  # Along the hallway the quickest way
                + 1                           # Into the room (first spot, underestimates)
            ) * ENERGY_COSTS[c]
            for c, points in self.amphipods.items()
            for p in points
            if p.x != TARGET_ROOMS[c]
        )


@app.command()
def main(initial_file: typer.FileText, goal_file: typer.FileText, heuristic: bool = False):
    initial = State.read(initial_file)
    goal = State.read(goal_file)

    q = queue.PriorityQueue()
    q.put((0, 0, initial))

    visited = set()

    i = 0
    while not q.empty():
        heuristic_score, energy, state = q.get_nowait()

        i += 1
        if i % 10000 == 0:
            logging.info(
                f'[{i=}, qsize={q.qsize()}] '
                f'heuristic={heuristic_score if heuristic else "disabled"} '
                f'{energy=} {str(state)}'
            )

        if state in visited:
            continue
        else:
            visited.add(state)

        if state == goal:
            break

        for new_energy, new_state in state.moves():
            if heuristic:
                heuristic_score = energy + new_energy + new_state.heuristic()
            else:
                heuristic_score = 0

            q.put((heuristic_score, energy + new_energy, new_state))

    print('Final solution:')
    print(state)
    print('states examined:', i)
    print(energy)


@app.callback()
def enableFlags(cache: bool = False, debug: bool = False):
    if debug:
        import coloredlogs  # type: ignore
        coloredlogs.install(level=logging.INFO)

    if cache:
        import functools
        global disableable_cache
        disableable_cache = functools.cache


if __name__ == '__main__':
    app()
