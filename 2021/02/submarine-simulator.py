import typer
import typing

app = typer.Typer()


@app.command()
def part1(commands: typer.FileText):
    position, depth = 0, 0

    for command in commands:
        key, svalue = command.split()
        value = int(svalue)

        if key == 'forward':
            position += value
        elif key == 'down':
            depth += value
        elif key == 'up':
            depth -= value

    print(f'{position=}, {depth=}, {position*depth=}')


@app.command()
def part2(commands: typer.FileText):
    position, depth, aim = 0, 0, 0

    for command in commands:
        key, svalue = command.split()
        value = int(svalue)

        if key == 'forward':
            position += value
            depth += aim * value
        elif key == 'down':
            aim += value
        elif key == 'up':
            aim -= value

    print(f'{position=}, {depth=}, {position*depth=}')


if __name__ == '__main__':
    app()
