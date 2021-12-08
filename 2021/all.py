import os
import time
import subprocess

problems = [
    [1, '01', 'Sonar Sweep', 'depth-finder.py', {
        'part1': 'input.txt',
        'part2': 'input.txt 3',
        'part2-simple': 'input.txt 3'
    }],
    [2, '02', 'Dive!', 'submarine-simulator.py', {
        'part1': 'input.txt',
        'part2': 'input.txt',
    }],
    [3, '03', 'Binary Diagnostic', 'binary-contraption.py', {
        'part1': 'input.txt',
        'part2': 'input.txt',
    }],
    [4, '04', 'Giant Squid', 'his-name-oh.py', {
        'part1': 'input.txt',
        'part2': 'input.txt',
    }],
    [5, '05', 'Hydrothermal Venture', 'linear-avoidinator.py', {
        'part1': 'input.txt',
        'part2': 'input.txt',
    }],
    [6, '06', 'Lanternfish', 'we-all-glow-down-here.py', {
        '80': 'input.txt',
        '256': 'input.txt',
    }],
    [7, '07', 'The Treachery of Whales', 'brachyura-aligner.py', {
        'part1': 'input.txt',
        'part2': 'input.txt',
    }],
    [8, '08', 'Seven Segment Search', 'seven-segment-demystifier.py', {
        'part1': 'input.txt',
        'part2': 'input.txt',
        '--fast part1': 'input.txt',
        '--fast part2': 'input.txt',
    }],
]

for day, folder, name, file, variants in problems:
    print(f'--- Day {day}: {name} ---\n')

    for cmd, args in variants.items():
        print(f'$ python3 {file} {cmd} {args}')

        start = time.perf_counter_ns()
        subprocess.check_call(f'python3 {file} {cmd} {args}', shell=True, cwd=folder)
        end = time.perf_counter_ns()

        print(f'# time {end-start}ns / {(end-start)/1e9:.2f}s\n')
