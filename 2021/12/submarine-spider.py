import typer

from dataclasses import dataclass
from functools import cache
from typing import *

app = typer.Typer()


@dataclass(frozen=True)
class Node:
    label: str

    def is_small(self):
        return self.label.islower()

    def __repr__(self):
        return f'<{self.label}>'


START = Node('start')
END = Node('end')


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

    def paths(node: Node, visited: Set[Node]) -> Generator[List[Node], None, None]:
        '''Yield all possible paths from the given node to <end>.'''

        if node == END:
            yield [END]
            return

        if node.is_small() and node in visited:
            return

        for next in cave.edges[node]:
            for path in paths(next, visited | {node}):
                yield [node] + path

    count = 0
    for _ in paths(START, set()):
        count += 1

    print(count)


@app.command()
def part2(file: typer.FileText):
    cave = Cave.from_file(file)

    def paths(node: Node, visited: Set[Node], used_double: bool) -> Generator[List[Node], None, None]:
        '''Yield all possible paths from the given node to <end>.'''

        if node == END:
            yield [END]
            return

        if node == START and START in visited:
            return

        if node.is_small() and node in visited and used_double:
            return

        for next in cave.edges[node]:
            for path in paths(next, visited | {node}, used_double or (node.is_small() and node in visited)):
                yield [node] + path

    count = 0
    for _ in paths(START, set(), False):
        count += 1

    print(count)


@app.command()
def part2_fast(file: typer.FileText):
    cave = Cave.from_file(file)

    @cache
    def paths(node: Node, visited: FrozenSet[Node], used_double: bool) -> List[List[Node]]:
        '''Yield all possible paths from the given node to <end>.'''

        if node == END:
            return [[END]]

        if node == START and START in visited:
            return []

        if node.is_small() and node in visited and used_double:
            return []

        return [
            [node] + path
            for next in cave.edges[node]
            for path in paths(next, visited | {node}, used_double or (node.is_small() and node in visited))
        ]

    count = 0
    for _ in paths(START, frozenset(), False):
        count += 1

    print(count)


if __name__ == '__main__':
    app()
