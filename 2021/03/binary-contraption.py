import copy
import typer

from typing import List

app = typer.Typer()


@app.command()
def part1(lines: typer.FileText):
    # Keep track of how many lines we counted total and just the number of ones
    counter = 0
    one_counts: List[int] = []

    for line in lines:
        line = line.strip()
        if not line:
            continue

        counter += 1

        # If we don't have an initalized counts, start with '0' or '1'
        if not one_counts:
            one_counts = list(map(int, line))

        # Otherwise, add for any ones
        else:
            for i, bit in enumerate(line):
                if int(bit) == 1:
                    one_counts[i] += 1

    # Recombine into a binary number, that is the gamma rate
    # The epsilon rate is the inverse
    gamma = ''.join(
        '1' if bit > counter / 2 else '0'
        for bit in one_counts
    )
    epsilon = ''.join(
        '1' if bit == '0' else '0'
        for bit in gamma
    )

    print(f'{gamma=}={int(gamma, 2)}, {epsilon=}={int(epsilon, 2)}, product={int(gamma, 2)*int(epsilon, 2)}')


@app.command()
def part2(file: typer.FileText):
    # Load the entire file into memory this time
    lines = [
        line.strip()
        for line in file
        if line.strip()
    ]

    potential_generators = copy.copy(lines)
    potential_scrubbers = copy.copy(lines)

    # Helper to find the most/least common bit in a given position from a given list
    # Note, the >= here is necessary to break ties (resulting in 1 for most common, 0 for least)
    def most_common(data, position):
        one_count = sum(
            1 if (line[position] == '1') else 0
            for line in data
        )
        return '1' if (one_count >= len(data) / 2) else '0'

    def least_common(data, position):
        return '1' if most_common(data, position) == '0' else '0'

    # Helper to filter an iterable so that lines with line[position] = value are kept
    def filter_position_equals(data, position, value):
        return list(filter(lambda line: line[position] == value, data))

    # Iterate through the bits
    for position in range(len(lines[0])):
        if len(potential_generators) > 1:
            potential_generators = filter_position_equals(
                potential_generators,
                position,
                most_common(potential_generators, position)
            )

        if len(potential_scrubbers) > 1:
            potential_scrubbers = filter_position_equals(
                potential_scrubbers,
                position,
                least_common(potential_scrubbers, position)
            )

    generator = int(potential_generators[0], 2)
    scrubber = int(potential_scrubbers[0], 2)

    print(
        f'{potential_generators=}={generator}, {potential_scrubbers=}={scrubber}, product={generator*scrubber}')


if __name__ == '__main__':
    app()
