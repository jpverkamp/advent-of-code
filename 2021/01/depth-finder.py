import typer
import typing

app = typer.Typer()


@app.command()
def part1(data: typer.FileText):
    count = 0
    last_depth = None

    for line in data:
        current_depth = int(line)

        if last_depth and current_depth > last_depth:
            count += 1

        last_depth = current_depth

    print(count)


@app.command()
def part2(data: typer.FileText, window_size: typing.Optional[int] = typer.Argument(1)):
    # Convert all depths to ints
    depths = list(map(int, data))

    # Calculate each window (depth[...]) and the sum of depths for each window
    slices = [
        sum(depths[start: start + window_size])
        for start in range(len(depths) - window_size + 1)
    ]

    # Count if we have an increase (b > a) for each pair of depths
    print(sum(
        1 if b > a else 0
        for a, b
        in zip(slices, slices[1:])
    ))


if __name__ == '__main__':
    app()
