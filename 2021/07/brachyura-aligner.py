import typer

app = typer.Typer()


@app.command()
def part1(file: typer.FileText):

    positions = [
        int(value)
        for line in file
        for value in line.split(',')
    ]

    fuel, target = min(
        (
            sum(abs(position - target) for position in positions),
            target
        )
        for target in range(min(positions), max(positions)+1)
    )

    print(f'{target=}, {fuel=}')


@app.command()
def part2(file: typer.FileText):

    def t(n):
        return n * (n + 1) // 2

    positions = [
        int(value)
        for line in file
        for value in line.split(',')
    ]

    fuel, target = min(
        (
            sum(t(abs(position - target)) for position in positions),
            target
        )
        for target in range(min(positions), max(positions)+1)
    )

    print(f'{target=}, {fuel=}')


if __name__ == '__main__':
    app()
