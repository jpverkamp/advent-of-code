import os
import time
import subprocess

problems = [
    [1, '01', 'Sonar Sweep', 'depth-finder.py', {
        'part1': 'input.txt',
        'part2': 'input.txt 3',
        'part2-simple': 'input.txt 3'
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
