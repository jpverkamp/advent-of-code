import itertools
import typer

from typing import Generator, List, TextIO

app = typer.Typer()

# Seven segment dislay
#   0:      1:      2:      3:      4:
#  aaaa    ....    aaaa    aaaa    ....
# b    c  .    c  .    c  .    c  b    c
# b    c  .    c  .    c  .    c  b    c
#  ....    ....    dddd    dddd    dddd
# e    f  .    f  e    .  .    f  .    f
# e    f  .    f  e    .  .    f  .    f
#  gggg    ....    gggg    gggg    ....
#
#   5:      6:      7:      8:      9:
#  aaaa    aaaa    aaaa    aaaa    aaaa
# b    .  b    .  .    c  b    c  b    c
# b    .  b    .  .    c  b    c  b    c
#  dddd    dddd    ....    dddd    dddd
# .    f  e    f  .    f  e    f  .    f
# .    f  e    f  .    f  e    f  .    f
#  gggg    gggg    ....    gggg    gggg

SEGMENTS = [
    {'a', 'b', 'c', 'e', 'f', 'g'},
    {'c', 'f'},
    {'a', 'c', 'd', 'e', 'g'},
    {'a', 'c', 'd', 'f', 'g'},
    {'b', 'c', 'd', 'f'},
    {'a', 'b', 'd', 'f', 'g'},
    {'a', 'b', 'd', 'e', 'f', 'g'},
    {'a', 'c', 'f'},
    {'a', 'b', 'c', 'd', 'e', 'f', 'g'},
    {'a', 'b', 'c', 'd', 'f', 'g'},
]

ALPHABET = 'abcdefg'
MAPPINGS = [
    dict(zip(ALPHABET, ordering))
    for ordering in itertools.permutations(ALPHABET)
]


def load(file: TextIO) -> Generator[List[int], None, None]:
    '''
    Load an input file with a scrambled set of 7 segment displays than 4 output digits.'''

    for line in file:
        raw_inputs, raw_outputs = line.split(' | ')
        inputs = [set(input) for input in raw_inputs.split()]
        outputs = [set(output) for output in raw_outputs.split()]

        for mapping in MAPPINGS:
            if any(
                {mapping[v] for v in input} not in SEGMENTS
                for input in inputs
            ):
                continue

            yield [
                SEGMENTS.index({mapping[v] for v in output})
                for output in outputs
            ]


@app.command()
def part1(file: typer.FileText):
    print(sum(
        1 if digit in (1, 4, 7, 8) else 0
        for digits in load(file)
        for digit in digits
    ))


@app.command()
def part2(file: typer.FileText):

    print(sum(
        int(''.join(str(digit) for digit in digits))
        for digits in load(file)
    ))


if __name__ == '__main__':
    app()
