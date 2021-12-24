import logging
import typer

from collections import *
from dataclasses import dataclass
from typing import *

app = typer.Typer()


def disableable_cache(x): return x


Die = Generator[int, None, None]


def LoadedDie(sides: int) -> Die:
    while True:
        for i in range(sides):
            yield i + 1


@dataclass
class Game:
    '''Reprents a weird/simple game.'''

    die: Die
    size: int
    target: int

    current: int
    players: List[int]
    scores: List[int]

    roll_count: int

    def __init__(
            self,
            die: Die,
            size: Optional[int] = 10,
            target: Optional[int] = 1000,
            number_of_players: Optional[int] = 2,
            initial_spots: Optional[List[int]] = None,
    ):
        '''Create a game with various defaults.'''

        if number_of_players is None:
            number_of_players = 2

        self.die = die
        self.size = size or 10
        self.target = target or 1000
        self.current = 0
        self.players = initial_spots or [0 for i in range(number_of_players)]
        self.scores = [0 for i in range(number_of_players)]
        self.roll_count = 0

        if not number_of_players == len(self.players):
            raise Exception('If both number of players and initial spots are provided, they must match')

    def winner(self) -> Optional[int]:
        '''Return the current winner (or None if no winner)'''

        for i, score in enumerate(self.scores):
            if score >= self.target:
                return i + 1

        return None

    def update(self):
        '''Plays one round.'''

        if self.winner():
            logging.info('Cannot update, game is already over')
            return

        self.roll_count += 3
        rolls = [next(self.die), next(self.die), next(self.die)]
        new_space = self.players[self.current] + sum(rolls)
        while new_space > self.size:
            new_space -= self.size

        self.players[self.current] = new_space
        self.scores[self.current] += new_space

        logging.info(f'''\
Player {self.current+1} rolls {rolls} \
and moves to space {self.players[self.current]} \
for a total score of {self.scores[self.current]}.\
''')

        self.current = (self.current + 1) % len(self.players)


@app.command()
def part1(file: typer.FileText):

    p1 = int(file.readline().split()[-1])
    p2 = int(file.readline().split()[-1])

    game = Game(die=LoadedDie(100), initial_spots=[p1, p2])

    while not game.winner():
        game.update()

    winning_player = game.winner()

    print(
        winning_player, 'wins,',
        game.scores[1 if winning_player == 1 else 0] * game.roll_count
    )


@app.command()
def part2(file: typer.FileText):

    location1 = int(file.readline().split()[-1])
    location2 = int(file.readline().split()[-1])

    # State is a mapping of (current player, p1 location, p1 score, p2 location, p2 score): count
    state = {
        (1, location1, 0, location2, 0): 1
    }

    # Rolls are the number (out of 27) of each possible roll we could see
    rolls: MutableMapping[int, int] = {}
    for i in range(1, 4):
        for j in range(1, 4):
            for k in range(1, 4):
                sum = i + j + k

                if sum in rolls:
                    rolls[sum] += 1
                else:
                    rolls[sum] = 1

    # Wins is the number of times each player has won
    wins = [0, 0, 0]

    # Advance the state of all possible simulations until every one is finished
    # Since we'll always advance at least 3, this will take a maximum of 7 plays for each player (14 total rounds)
    while state:
        logging.info(f'{state=}, {wins=}')
        next_state = {}

        # Unpack all of the current states
        for (player, location1, score1, location2, score2), current_count in state.items():
            # Try every possible combination of rolls of the dice
            for roll, new_count in rolls.items():
                # Make sure we're working on the correct player
                (location, score) = (location1, score1) if player == 1 else (location2, score2)

                # Update that player's location and score
                new_location = location + roll
                while new_location > 10:
                    new_location -= 10

                score += new_location

                # If the score is >= 21, don't add those possibilities back into the state space
                # Instead, increment number of wins by the ways we got here (current_count) times the ways to advance (new_count)
                if score >= 21:
                    wins[player] += current_count * new_count
                    continue

                # If we didn't win, calculate the new index into the state variable
                # This basically means swapping the player and updating either location1,score1 or location2,score2
                new_index = (
                    2 if player == 1 else 1,
                    new_location if player == 1 else location1,
                    score if player == 1 else score1,
                    new_location if player == 2 else location2,
                    score if player == 2 else score2
                )

                # Update the counts in the next state by the ways we got here (current_count) times the ways to advance (new_count)
                if new_index not in next_state:
                    next_state[new_index] = 0
                next_state[new_index] += current_count * new_count

        state = next_state

    logging.info(f'Final {state=}, {wins=}')
    print(max(wins))


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
