import os
import time
import subprocess
import sys

problems = [
    [1, '01', 'Sonar Sweep', 'depth-finder.py', [
        'part1 input.txt',
        'part2 input.txt 3',
        'part2-simple input.txt 3'
    ]],
    [2, '02', 'Dive!', 'submarine-simulator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [3, '03', 'Binary Diagnostic', 'binary-contraption.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [4, '04', 'Giant Squid', 'his-name-oh.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [5, '05', 'Hydrothermal Venture', 'linear-avoidinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [6, '06', 'Lanternfish', 'we-all-glow-down-here.py', [
        '80 input.txt',
        '256 input.txt',
    ]],
    [7, '07', 'The Treachery of Whales', 'brachyura-aligner.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [8, '08', 'Seven Segment Search', 'seven-segment-demystifier.py', [
        'part1 input.txt',
        'part2 input.txt',
        '--fast part1 input.txt',
        '--fast part2 input.txt',
    ]],
    [9, '09', 'Smoke Basin', 'local-minimum-deminifier.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [10, '10', 'Syntax Scoring', 'chunkinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [11, '11', 'Dumbo Octopus', 'octopus-flashinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [12, '12', 'Passage Passing', 'submarine-spider.py', [
        'part1 input.txt',
        'part2 input.txt',
        'part2-fast input.txt',
    ]],
    [13, '13', 'Transparent Origami', 'foldinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [14, '14', 'Extended Polymerization', 'polymerizationinator.py', [
        'direct input.txt 10',
        'recursive input.txt 10',
        'direct input.txt 15',
        'recursive input.txt 15',
        '--cache recursive input.txt 15',
        '--cache recursive input.txt 40',
    ]],
    [15, '15', 'Chiton', 'low-ceiling-simulator.py', [
        'part1 input.txt',
        '--version 2 part1 input.txt',
        '--version 3 part1 input.txt',
        '--version 4 part1 input.txt',
        '--version 4 part2 input.txt',
    ]],
    [16, '16', 'Packet Decoder', 'depacketinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [17, '17', 'Trick Shot', 'pew-pewinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [18, '18', 'Snailfish', 'pairs-of-pairs.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [19, '19', 'Beacon Scanner', 'point-matchinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [20, '20', 'Trench Map', 'enhancinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [21, '21', 'Dirac Dice', 'dicinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [22, '22', 'Reactor Reboot', 'cubinator.py', [
        'part1 input.txt',
        'part2 input.txt',
    ]],
    [23, '23', 'Amphipods', 'amphipodinator.py', [
        'main input1.txt goal1.txt',
        'main input1.txt goal1.txt --heuristic',
        'main input2.txt goal2.txt --heuristic',
    ]],
    [24, '24', 'Arithmetic Logic Unit', 'aluinator.py', [
        'solve input.txt',
    ]],
    [25, '25', 'Sea Cucumber', 'cucumbinator.py', [
        'solve input.txt',
    ]]
]

# If any numbers are specified on the command line
if len(sys.argv) > 1:
    problems = [
        problem
        for problem in problems
        if problem[1] in sys.argv
    ]

for day, folder, name, file, variants in problems:
    print(f'--- Day {day}: {name} ---\n')

    for args in variants:
        print(f'$ python3 {file} {args}')

        try:
            start = time.perf_counter_ns()
            subprocess.check_call(f'python3 {file} {args}', shell=True, cwd=folder,
                                  timeout=60.0 if '--timeout' in sys.argv else None)
            end = time.perf_counter_ns()
            print(f'# time {end-start}ns / {(end-start)/1e9:.2f}s\n')

        except subprocess.TimeoutExpired:
            print('# Process timed out after 1 minute\n')
