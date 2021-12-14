import typer
import os
import pathlib

app = typer.Typer()

template = '''\
import typer

from collections import *
from dataclasses import dataclass
from functools import cache
from typing import *

app = typer.Typer()

@app.command()
def part1(file: typer.FileText):
    pass

@app.command()
def part2(file: typer.FileText):
    pass

if __name__ == '__main__':
    app()
'''


@app.command()
def main(day: int, name: str):
    padded_day = f'{day:02d}'

    os.makedirs(padded_day, exist_ok=True)

    pathlib.Path(os.path.join(padded_day, 'test.txt')).touch()
    pathlib.Path(os.path.join(padded_day, 'input.txt')).touch()

    with open(os.path.join(padded_day, name + ('' if name.endswith('.py') else '.py')), 'w') as f:
        f.write(template)


if __name__ == '__main__':
    app()
