import logging
import typer

from collections import *
from dataclasses import dataclass
from typing import *
from pprint import pprint, pformat

app = typer.Typer()


def disableable_cache(x): return x


PRE = '      '
ALL_INPUTS = set(range(10))
OPS = {
    'add': lambda a, b: a + b,
    'mul': lambda a, b: a * b,
    'div': lambda a, b: a // b,
    'mod': lambda a, b: a % b,
    'eql': lambda a, b: 1 if a == b else 0,
    'neq': lambda a, b: 1 if a != b else 0,
}


@dataclass(frozen=True)
class Node:
    pass

    @disableable_cache
    def partial_eval(self) -> 'Node':
        return self


@dataclass(frozen=True)
class Literal(Node):
    value: int

    def to_json(self):
        return self.value

    def __repr__(self):
        return f'{self.value}'

    @disableable_cache
    def __call__(self, input: str, depth: int = 0) -> int:
        return self.value


@dataclass(frozen=True)
class Input(Node):
    index: int

    def to_json(self):
        return ['I', self.index]

    def __repr__(self):
        return f'I({self.index})'

    @disableable_cache
    def __call__(self, input: str, depth: int = 0) -> int:
        result = int(input[self.index])
        logging.debug(f'{PRE}{" " * depth}I({self.index}) -> {result}')
        return result


@dataclass(frozen=True)
class Operator(Node):
    function: str
    left: Node
    right: Node

    def to_json(self):
        return [self.function, self.left.to_json(), self.right.to_json()]

    @disableable_cache
    def __call__(self, input: str, depth: int = 0) -> int:
        logging.debug(f'{PRE}{" " * depth}{self}')

        l_value = self.left(input, depth + 1)
        r_value = self.right(input, depth + 1)
        result = OPS[self.function](l_value, r_value)

        logging.debug(f'{PRE}{" " * depth}{self.function}({l_value}, {r_value}) -> {result}')

        return result

    @disableable_cache
    def partial_eval(self) -> 'Node':
        l_value = self.left.partial_eval()
        r_value = self.right.partial_eval()

        # Literal evals can be directly simplified
        if isinstance(l_value, Literal) and isinstance(r_value, Literal):
            try:
                result = Literal(OPS[self.function](l_value.value, r_value.value))
            except ZeroDivisionError:
                result = Literal(0)

            logging.info(f'{PRE} Simplifying {self} -> {result}')
            return result

        # additive identity
        elif self.function == 'add' and isinstance(l_value, Literal) and l_value.value == 0:
            logging.info(f'{PRE} Applying 0+a=a')
            return r_value

        elif self.function == 'add' and isinstance(r_value, Literal) and r_value.value == 0:
            logging.info(f'{PRE} Applying a+0=a')
            return l_value

        # multiplicative identity
        elif self.function == 'mul' and isinstance(l_value, Literal) and l_value.value == 1:
            logging.info(f'{PRE} Applying 1*a=a')
            return r_value

        elif self.function == 'mul' and isinstance(r_value, Literal) and r_value.value == 1:
            logging.info(f'{PRE} Applying a*1=a')
            return l_value

        elif self.function == 'mul' and isinstance(l_value, Literal) and l_value.value == 0:
            logging.info(f'{PRE} Applying 0*a=a')
            return Literal(0)

        elif self.function == 'mul' and isinstance(r_value, Literal) and r_value.value == 0:
            logging.info(f'{PRE} Applying a*0=a')
            return Literal(0)

        # division identities
        elif self.function == 'div' and isinstance(r_value, Literal) and r_value.value == 1:
            logging.info(f'{PRE} Applying a/1=a')
            return l_value

        elif self.function == 'div' and l_value == r_value:
            logging.info(f'{PRE} Applying a/a=1')
            return Literal(1)

        # equality identities
        elif self.function == 'eql' and l_value == r_value:
            logging.info(f'{PRE} Applying a == a -> 1')
            return Literal(1)

        elif (
            self.function == 'eql'
            and isinstance(l_value, Operator)
            and l_value.function == 'eql'
            and isinstance(r_value, Literal)
            and r_value.value == 0
        ):
            result = Operator('neq', l_value.left, l_value.right)
            logging.info(f'{PRE} Converting eq to neq')
            return result

        # If we're doing EQL any input to a value not [0, 9], that's always 0
        elif (
            self.function == 'eql'
            and isinstance(l_value, Literal)
            and (l_value.value < 0 or l_value.value > 9)
            and isinstance(r_value, Input)
        ):
            logging.info(f'{PRE} Simplifying impossible {self} -> 0')
            return Literal(0)

        elif (
            self.function == 'eql'
            and isinstance(r_value, Literal)
            and (r_value.value < 0 or r_value.value > 9)
            and isinstance(l_value, Input)
        ):
            logging.info(f'{PRE} Simplifying impossible {self} -> 0')
            return Literal(0)

        elif (
            self.function == 'neq'
            and isinstance(l_value, Literal)
            and (l_value.value < 0 or l_value.value > 9)
            and isinstance(r_value, Input)
        ):
            logging.info(f'{PRE} Simplifying impossible {self} -> 0')
            return Literal(1)

        elif (
            self.function == 'neq'
            and isinstance(r_value, Literal)
            and (r_value.value < 0 or r_value.value > 9)
            and isinstance(r_value, Input)
        ):
            logging.info(f'{PRE} Simplifying impossible {self} -> 0')
            return Literal(1)

        # Base case
        else:
            return self

    def __repr__(self):
        return f'{self.function}({self.left}, {self.right})'


def parse(file: TextIO) -> Node:
    '''Read input into a list of instructions.'''

    registers = {
        'w': Literal(0),
        'x': Literal(0),
        'y': Literal(0),
        'z': Literal(0),
    }

    next_input_index = 0

    for i, line in enumerate(file, 1):
        line = line.strip()
        logging.info(f'[{i:04d}] Parsing {line}')

        cmd, *args = line.strip().split()
        a = args[0]

        if cmd == 'inp':
            registers[a] = Input(next_input_index)
            next_input_index += 1
            continue

        b = args[1]
        if not b.isalpha():
            right = Literal(int(b))
        else:
            right = registers[b]

        registers[a] = Operator(cmd, registers[a], right).partial_eval()
        #logging.info(f'\n{pformat(registers)}')

        # TODO: DEBUG
        #if i > 150:
        #    break

    return registers['z']


def flatten(s: frozenset[int]) -> int:
    return list(s)[0]


@app.command()
def part1(file: typer.FileText):

    code = parse(file)
    pprint(code.to_json())
    return

    i = 99999999999999
    while True:
        istr = str(i)
        if '0' in istr:
            continue

        result = code(istr)
        logging.info(f'{i}: {result}')

        if result == 0:
            break

        i -= 1

    print(i)


@app.command()
def part2(file: typer.FileText):
    pass


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
