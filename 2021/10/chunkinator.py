import typer

from dataclasses import dataclass
from typing import *

app = typer.Typer()

# Using the strings directly is fine for python, but not mypy
# chunkinator.py:38: error: Unpacking a string is disallowed
pairs = [tuple(pair) for pair in ['()', '[]', '{}', '<>']]


@dataclass
class ParseMismatchException(Exception):
    expected: str
    actual: str


@dataclass
class ParseIncompleteException(Exception):
    remaining: str


def parse(line: str):
    '''
    Parse a line of matching pairs. For every left, you must match the corresponding right (respecting nesting).

    If a mismatch is detected, raise a ParseMismatchException containing the expected/actual character.
    If the parse is incomplete, raise a ParseIncomplete exception with the necessary characters to finish the parse.

    Otherwise, return True
    '''

    stack = []

    for c in line:
        matched = False

        # Start a new matching pair
        for left, right in pairs:
            if c == left:
                stack.append(right)
                matched = True
        if matched:
            continue

        # Otherwise, we have a closing character, check the stack
        if c == (top := stack.pop()):
            continue

        # Otherwise, we have a failed match
        raise ParseMismatchException(expected=top, actual=c)

    # If we still have something left to parse, notify
    if stack:
        raise ParseIncompleteException(remaining=''.join(reversed(stack)))

    # Otherwise, yay!
    return True


@app.command()
def part1(file: typer.FileText):

    scores = {')': 3, ']': 57, '}': 1197, '>': 25137}
    total_score = 0

    for line in file:
        line = line.strip()

        try:
            parse(line)

        except ParseMismatchException as ex:
            total_score += scores[ex.actual]

        except ParseIncompleteException:
            pass

    print(total_score)


@app.command()
def part2(file: typer.FileText):

    scores = {')': 1, ']': 2, '}': 3, '>': 4}
    line_scores = []

    for line in file:
        line = line.strip()

        try:
            parse(line)

        except ParseMismatchException:
            pass

        except ParseIncompleteException as ex:
            line_score = 0
            for c in ex.remaining:
                line_score = line_score * 5 + scores[c]

            line_scores.append(line_score)

    print(sorted(line_scores)[len(line_scores) // 2])


if __name__ == '__main__':
    app()
