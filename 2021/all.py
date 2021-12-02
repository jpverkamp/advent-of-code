import os
import time
import subprocess

problems = [
    [1, '01', 'Sonar Sweep', 'depth-finder.py', 'input.txt', 'input.txt 3'],
]

for day, folder, name, cmd, part1, part2 in problems:
    print(f'--- Day {day}: {name} ---')

    for cmd in [f'python3 {cmd} part1 {part1}', f'python3 {cmd} part2 {part2}']:
        print(f'$ {cmd}')

        start = time.perf_counter_ns()
        subprocess.check_call(cmd, shell=True, cwd=folder)
        end = time.perf_counter_ns()

        print(f'# time {end-start}ns / {(end-start)/1e9:.2f}s\n')
