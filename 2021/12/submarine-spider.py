import typer

from dataclasses import dataclass
from functools import cache
from typing import *

app = typer.Typer()


@dataclass(frozen=True)
class Node:
    label: str

    def is_big(self):
        return self.label.isupper()

    def __repr__(self):
        return f'<{self.label}>'


@dataclass(frozen=True)
class Cave:
    edges: Mapping[Node, Set[Node]]

    @staticmethod
    def from_file(file: TextIO):

        edges: Dict[Node, Set[Node]] = {}

        for line in file:
            nodes = [Node(v) for v in line.strip().split('-')]

            for node in nodes:
                if node not in edges:
                    edges[node] = set()

            for node_a in nodes:
                for node_b in nodes:
                    if node_a != node_b:
                        edges[node_a].add(node_b)

        # Convert to a dict
        return Cave(edges)


@app.command()
def part1(file: typer.FileText):
    cave = Cave.from_file(file)

    def paths(node: Node, visited: List[Node]):
        '''Yield all possible paths from the given node to <end>.'''

        # Default to the start Node and no visited nodes
        if node is None:
            node = Node('start')

        if visited is None:
            visited = [node]

        # If we're at the end node, generate the collected path
        if node == Node('end'):
            yield visited
            return

        # Otherwise, try each next step
        for next in cave.edges[node]:
            # Cannot visit small nodes that we've already seen
            if next in visited and not next.is_big():
                continue

            # Otherwise, generate all paths down that route
            yield from paths(next, visited + [next])

    start = Node('start')

    count = 0
    for _ in paths(start, [start]):
        count += 1

    print(count)


@app.command()
def part2(file: typer.FileText):
    cave = Cave.from_file(file)

    def paths(node: Node, visited: List[Node]):
        '''Yield all possible paths from the given node to <end>.'''

        # Default to the start Node and no visited nodes
        if node is None:
            node = Node('start')

        if visited is None:
            visited = [node]

        # If we're at the end node, generate the collected path
        if node == Node('end'):
            yield visited
            return

        # Otherwise, try each next step
        for next in cave.edges[node]:
            # Can visit big nodes any number of times
            if next.is_big():
                pass

            # Can visit small nodes we haven't already visited
            elif next not in visited:
                pass

            # Cannot visit <start> twice
            elif next == Node('start'):
                continue

            # We're trying to visit the same small node twice
            # And we've already visited a small node twice
            elif any(not n.is_big() and visited.count(n) > 1 for n in visited):
                continue

            # Otherwise, generate all paths down that route
            yield from paths(next, visited + [next])

    start = Node('start')

    count = 0
    for _ in paths(start, [start]):
        count += 1

    print(count)


if __name__ == '__main__':
    app()
