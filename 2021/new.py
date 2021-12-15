import typer
import os
import pathlib

app = typer.Typer()

template = '''\
import logging
import typer

from collections import *
from dataclasses import dataclass
from typing import *

app = typer.Typer()

def disableable_cache(x): return x

@app.command()
def part1(file: typer.FileText):
    pass

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
