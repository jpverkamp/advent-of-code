import logging
import typer
import time

from collections import *
from dataclasses import dataclass
from typing import *
from pprint import pprint, pformat

app = typer.Typer()


def disableable_cache(x): return x


PRE = '      '
ALL_INPUTS = set(range(1, 10))
OPS = {
    'add': lambda a, b: a + b,
    'mul': lambda a, b: a * b,
    'div': lambda a, b: a // b,
    'mod': lambda a, b: a % b,
    'eql': lambda a, b: 1 if a == b else 0,
    'neq': lambda a, b: 1 if a != b else 0,
}


@dataclass(frozen=True)
class State:
    w: int
    x: int
    y: int
    z: int

    def __getitem__(self, key: Union[str, int]) -> int:
        if key in 'wxyz':
            return getattr(self, key)

        return int(key)

    def set(self, key: str, value: int) -> 'State':
        return State(
            value if key == 'w' else self.w,
            value if key == 'x' else self.x,
            value if key == 'y' else self.y,
            value if key == 'z' else self.z,
        )

    def __repr__(self):
        return f'{{{self.w}, {self.x}, {self.y}, {self.z}}}'


def read(file: TextIO):
    start = time.perf_counter()
    states = {State(0, 0, 0, 0): ('', '')}

    input_length = 0
    for i, line in enumerate(file, 1):
        line = line.strip()
        logging.info(
            f'[line={i:04d} len={input_length} states={len(states)} time={time.perf_counter() - start:02f}s] {line}')

        cmd, *args = line.split()

        if cmd == 'inp':
            # w/x/y are always reset between inp blocks, so eliminate all current states that don't do that now
            # Note: This doesn't seem to actually help since this is done anyways
            if False:
                new_states = {}
                for state, (min_input, max_input) in states.items():
                    new_state = State(0, 0, 0, state.z)

                    if new_state not in new_states:
                        new_states[new_state] = (min_input, max_input)

                    old_min_input, old_max_input = new_states[new_state]

                    new_states[new_state] = (
                        min(min_input, old_min_input),
                        max(max_input, old_max_input),
                    )

                states = new_states

            # Now expand again with the new inputs
            a = args[0]
            new_states = {}
            input_length += 1

            for i in ALL_INPUTS:
                for state, (min_input, max_input) in states.items():
                    new_state = state.set(a, i)
                    new_min_input = min_input + str(i)
                    new_max_input = max_input + str(i)

                    if new_state not in new_states:
                        new_states[new_state] = (new_min_input, new_max_input)

                    new_states[new_state] = (
                        min(min_input, new_min_input),
                        max(max_input, new_max_input),
                    )

            states = new_states

        else:
            a, b = args
            states = {
                state.set(a, OPS[cmd](state[a], state[b])): input
                for state, input in states.items()
            }

    min_result = None
    max_result = None

    logging.info('Scanning for final solution')
    for state, (min_input, max_input) in states.items():
        if state.z != 0:
            continue

        if not min_result or min_input < min_result:
            logging.info(f'New minimum found: {min_input}')
            min_result = min_input

        if not max_result or max_input > max_result:
            logging.info(f'New maximum found: {min_input}')
            max_result = max_input

    return (min_result, max_result)


@app.command()
def part1(file: typer.FileText):

    min_result, max_result = read(file)
    print(min_result, max_result)


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
