import typer

from typing import List, TextIO, MutableMapping

app = typer.Typer()


class School:
    '''Represents a school of fish.'''

    def __init__(self, fish):
        '''Loads the given list of fish into a map of ages.'''

        self.data = {
            age: 0
            for age in range(9)
        }

        for each in fish:
            self.data[each] += 1

    @staticmethod
    def from_file(file: TextIO):
        '''Load a school from a file-like object'''

        fish = [
            int(age)
            for line in file
            for age in line.split(',')
        ]

        return School(fish)

    def step(self):
        '''
        Advance this school 1 day.

        All fish increase in age by 1 tick
        Fish that are of a spawning age reset to 7 days to spawn and create a new 9 day to spawn fish.

        Remember 0 based indexing.
        '''

        # Remember how many fish are going to spawn
        breeding = self.data[0]

        # Increase age of each fish by 1
        for age in range(1, 9):
            self.data[age - 1] = self.data[age]

        # Each fish that spawns moves to age 6 (don't overwrite previously age 7) and spawns a new one of age 8
        self.data[6] += breeding
        self.data[8] = breeding

    def __str__(self):
        '''Return a comma-delimited list of fish ages.'''

        return ','.join(
            str(age)
            for age, qty in self.data.items()
            for _ in range(qty)
        )

    def __len__(self):
        '''Return the number of fish in the school.'''

        return sum(qty for age, qty in self.data.items())

    def size(self):
        '''Return the number of fish in the school (__len__ is limited to an index-sized integer).'''

        return sum(qty for age, qty in self.data.items())


def load(file: TextIO) -> School:
    return School(int(age) for age in file.readline().split(','))


@app.command()
def main(ticks: int, file: typer.FileText):
    school = School.from_file(file)

    for i in range(ticks):
        school.step()

    print(school.size())

    size_string = str(school.size())
    print(f'{size_string[0]}.{size_string[1:3]}e{len(size_string)}')


if __name__ == '__main__':
    app()
