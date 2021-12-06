import itertools
import typer

from typing import Tuple, List, Optional, TextIO

app = typer.Typer()

BingoBoard = List[List[Optional[int]]]


def parse(file: TextIO) -> Tuple[List[int], List[BingoBoard]]:
    '''Parse a bingo definition. Return the list of numbers (as ints) and a list of 5x5 grids.'''

    numbers = [int(el) for el in file.readline().split(',')]
    file.readline()

    boards: List[BingoBoard] = []
    board: BingoBoard = []

    for line in file:
        if not line.strip():
            continue

        board.append([int(el) for el in line.split()])

        if len(board) == 5:
            boards.append(board)
            board = []

    return numbers, boards


def check_off(board: BingoBoard, number: int):
    '''Remove the given number from the given board by changing it to None.'''

    # Yes, I know I'm hardcoding 5
    for i in range(5):
        for j in range(5):
            if board[i][j] == number:
                board[i][j] = None


def is_solved(board):
    '''Return True if any row or column is completely None.'''

    # This is silly looking
    return (
        any(
            all(board[i][j] is None for j in range(5))
            for i in range(5)
        ) or
        any(
            all(board[i][j] is None for i in range(5))
            for j in range(5)
        )
    )


@app.command()
def part1(file: typer.FileText):

    numbers, boards = parse(file)

    for number in numbers:
        for board in boards:
            check_off(board, number)

            # As soon as any board is solved, we're done
            if is_solved(board):
                remaining_numbers = [el for row in board for el in row if el]
                print(f'{remaining_numbers=}, {sum(remaining_numbers)=}, {number=}, product={sum(remaining_numbers)*number}')
                return


@app.command()
def part2(file: typer.FileText):

    numbers, boards = parse(file)

    for number in numbers:
        for board in boards:
            check_off(board, number)

        # If (and only if) we're on the last board, check for a solution and bail
        if len(boards) == 1 and (board := boards[0]) and is_solved(board):
            remaining_numbers = [el for row in board for el in row if el]
            print(f'{remaining_numbers=}, {sum(remaining_numbers)=}, {number=}, product={sum(remaining_numbers)*number}')
            return

        # Otherwise, remove solved boards
        boards = [board for board in boards if not is_solved(board)]


if __name__ == '__main__':
    app()
