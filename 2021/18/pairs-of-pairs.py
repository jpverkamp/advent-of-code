import logging
import math
import typer

from collections import *
from dataclasses import dataclass
from typing import *

app = typer.Typer()


def disableable_cache(x): return x


@dataclass(frozen=True)
class Snailfish():
    left: Union[int, 'Snailfish']
    right: Union[int, 'Snailfish']

    @staticmethod
    def read(text: TextIO) -> Optional[Union[int, 'Snailfish']]:
        '''Read a snailfish from the given filelike object'''

        c = text.read(1)
        while not c.isdigit() and not c in '[':
            c = text.read(1)

        if c == '[':
            left = Snailfish.read(text)
            right = Snailfish.read(text)

            assert left is not None
            assert right is not None

            return Snailfish(left, right)

        elif c.isdigit():
            the_once_and_future_number = []

            while c.isdigit():
                the_once_and_future_number.append(c)
                c = text.read(1)

            assert(c == ',' or c == ']')

            return int(''.join(the_once_and_future_number))

        return None

    def to_depthlist(self) -> List[Tuple[int, int]]:
        '''Convert to list of (value, depth).'''

        def g(sf: Union[int, 'Snailfish'], depth: int) -> Generator[Tuple[int, int], None, None]:
            if isinstance(sf, int):
                yield (sf, depth)
            else:
                yield from g(sf.left, depth + 1)
                yield from g(sf.right, depth + 1)

        return list(g(self, 0))

    @staticmethod
    def from_depthlist(dls: List[Tuple[int, int]]) -> 'Snailfish':
        '''Convert from a list of (value, depth).'''

        # To make typing happy, copy to a list that can have either
        mixedls: List[Tuple[Union[int, 'Snailfish'], int]] = [
            (value, depth)
            for (value, depth)
            in dls
        ]

        while len(mixedls) > 1:
            for index, ((left, left_depth), (right, right_depth)) in enumerate(zip(mixedls, mixedls[1:])):
                if left_depth == right_depth:
                    mixedls[index] = (Snailfish(left, right), left_depth - 1)
                    del mixedls[index+1]

                    break

        assert isinstance(mixedls[0][0], Snailfish)
        return mixedls[0][0]

    def reduce(self) -> 'Snailfish':
        '''Convert this snailfish to minimum form using the result for explosing and splitting.'''

        # Much easier to work with mutable depthlists...
        dls = self.to_depthlist()

        reducing = True
        while reducing:
            reducing = False
            logging.debug(f'Reducing: dls={dls}, sf={Snailfish.from_depthlist(dls)}')

            # Check for any pairs that needs exploding
            for index, (value, depth) in enumerate(dls):
                if depth > 4 and index < len(dls) - 1 and depth == dls[index + 1][1]:
                    logging.debug(f' - Exploding at {index=} with {value=} and {depth=}')

                    prefix = dls[:index]
                    infix = [(0, depth-1)]
                    suffix = dls[index+2:]

                    # Increase the previous value by one (if it exists)
                    if prefix:
                        prefix[-1] = (prefix[-1][0] + value, prefix[-1][1])

                    # Increase the next value by one (if it exists)
                    if suffix:
                        suffix[0] = (suffix[0][0] + dls[index+1][0], suffix[0][1])

                    dls = prefix + infix + suffix

                    reducing = True
                    break

            if reducing:
                continue

            # If not exploding, check for any value that needs splitting
            for index, (value, depth) in enumerate(dls):
                if value >= 10:
                    logging.debug(f' - Splitting at {index=} with {value=}')

                    dls = (
                        dls[:index]
                        + [
                            (math.floor(value / 2), depth + 1),
                            (math.ceil(value / 2), depth + 1),
                        ]
                        + dls[index+1:]
                    )

                    reducing = True
                    break

        return Snailfish.from_depthlist(dls)

    def magnitude(self) -> int:
        '''Calculate the magnitude of a snailfish number.'''

        def f(sf: Union[int, 'Snailfish']) -> int:
            if isinstance(sf, int):
                return sf
            else:
                return 3 * f(sf.left) + 2 * f(sf.right)

        return f(self)

    def __add__(self, other):
        '''Add two Snailfish by making a larger pair and then reducing it.'''

        return Snailfish(self, other).reduce()

    def __repr__(self):
        return f'{{{self.left}, {self.right}}}'


@app.command()
def part1(file: typer.FileText):

    sum: Optional[Snailfish] = None

    while sf := Snailfish.read(file):
        logging.info(f'Adding: {sf} to {sum}')
        assert isinstance(sf, Snailfish)

        if sum is None:
            sum = sf
        else:
            sum += sf

    assert sum is not None

    logging.info(f'Final result: {sum}')
    print(sum.magnitude())


@app.command()
def part2(file: typer.FileText):

    sfes: List[Snailfish] = []
    while sf := Snailfish.read(file):
        assert isinstance(sf, Snailfish)
        sfes.append(sf)

    print(max(
        (sf1 + sf2).magnitude()
        for sf1 in sfes
        for sf2 in sfes
        if sf1 != sf2
    ))


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
