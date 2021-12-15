import logging
import typer

from collections import *
from dataclasses import dataclass
from typing import *

app = typer.Typer()


def disableable_cache(x): return x


PolyMap = Mapping[Tuple[str, str], str]



@dataclass
class Node(Iterable):
    value: str
    next: Optional['Node']

    def expand_via(self, map: PolyMap) -> 'Node':
        if self.next is None:
            return self

        for a, b in zip(self, self.next):
            if (a.value, b.value) in map:
                a.next = Node(map[a.value, b.value], b)

        return self

    @staticmethod
    def from_iter(iter: Iterable):
        head = None
        previous = None

        for el in iter:
            n = Node(el, None)

            if previous:
                previous.next = n
            else:
                head = n

            previous = n

        return head

    def __iter__(self):
        self.iter_node = self
        return self

    def __next__(self):
        if not self.iter_node:
            raise StopIteration

        result = self.iter_node
        self.iter_node = self.iter_node.next
        return result

    def __str__(self):
        return ''.join(self)


def load(file: TextIO) -> Tuple[Node, PolyMap]:

    ls = Node.from_iter(file.readline().strip())
    file.readline()

    map = {
        (line[0], line[1]): line[6]
        for line in file
    }

    return ls, map


@app.command()
def direct(file: typer.FileText, steps: int):

    ls, map = load(file)

    import time
    start = time.time()

    for i in range(steps):
        logging.info(f'Generation {i} calculate after {time.time() - start:02f} sec, has {len(list(ls))} elements')
        ls.expand_via(map)

    counts = Counter(n.value for n in ls)
    logging.info(f'{counts=}')

    most, _ = max((qty, c) for (c, qty) in counts.items())
    least, _ = min((qty, c) for (c, qty) in counts.items())

    print(most - least)


@app.command()
def recursive(file: typer.FileText, steps: int):

    ls, map = load(file)

    # Recursively count all elements that will be returned from the character pair a,b at depth
    # This will use the mapping specified in map above
    # This will recur depth += 1 each time until depth = steps (so will always terminate)
    # The @cache will make sure that we only recalculate a given a/b/depth triple once
    @disableable_cache
    def count(a, b, depth):
        logging.info(f'{" " * depth} > count({a}, {b}, {depth})')

        if depth == steps:
            result = {a: 1}
        else:
            result = {}

            for left, right in [(a, map[a, b]), (map[a, b], b)]:
                for k, v in count(left, right, depth + 1).items():
                    result[k] = result.get(k, 0) + v

        logging.info(f'{" " * depth} < count({a}, {b}, {depth}) = {result}')
        return result

    # Recursively figure out the counts for each pair of elements
    # The rightmost element is never counted, so add it at the end
    counts: MutableMapping[str, int] = {}
    if ls.next is not None:
        for a, b in zip(ls, ls.next):
            for k, v in count(a.value, b.value, 0).items():
                counts[k] = counts.get(k, 0) + v
        counts[b.value] = counts.get(b.value, 0) + 1

    logging.info(f'{counts=}')

    most, _ = max((qty, c) for (c, qty) in counts.items())
    least, _ = min((qty, c) for (c, qty) in counts.items())

    print(most - least)


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
