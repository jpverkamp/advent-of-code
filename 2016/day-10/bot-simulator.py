#!/usr/bin/env python3

import argparse
import regex as re

parser = argparse.ArgumentParser()
parser.add_argument('input')
parser.add_argument('targets', type = int, nargs = 2, help = 'values to record as being shared')
parser.add_argument('--debug', action = 'store_true', default = False)
args = parser.parse_args()

class Bot(object):
    cache = {}

    def __init__(self, name):
        self.name = name
        self.values = set()
        self.low_output = None
        self.high_output = None
        self.compared = []

        if args.debug:
            print('{} created'.format(self))

    @staticmethod
    def get(name):
        if not name in Bot.cache:
            Bot.cache[name] = Bot(name)

        return Bot.cache[name]

    @staticmethod
    def all():
        for name in sorted(list(Bot.cache.keys())):
            yield Bot.get(name)

    def __str__(self):
        return 'Bot<{name}, {values}, low:{low_output}, high:{high_output}>'.format(
            name = self.name,
            values = list(sorted(self.values)),
            low_output = self.low_output,
            high_output = self.high_output,
        )

    def give(self, value):
        if args.debug:
            print('{} given {}'.format(self, value))

        self.values.add(value)

        if len(self.values) == 2:
            self.compared.append(set(self.values))

            if not self.low_output or not self.high_output:
                #raise Exception('{} got a second value but has not output'.format(self))
                print('{} got a second value but has not output'.format(self))
                return

            Bot.get(self.low_output).give(min(self.values))
            Bot.get(self.high_output).give(max(self.values))

            self.values.clear()

re_value = re.compile(r'value (?P<value>\d+) goes to (?P<name>(?:(bot|output)) (\d+))')
re_mapping = re.compile(r'(?P<input>(?:(bot|output)) (\d+)) gives low to (?P<low_output>(?:(bot|output)) (\d+)) and high to (?P<high_output>(?:(bot|output)) (\d+))')

if args.debug:
    print('-- running simulation --')

values = list()

with open(args.input, 'r') as fin:
    for line in fin:
        if args.debug:
            print(line.strip())

        m_value = re_value.match(line)
        m_mapping = re_mapping.match(line)

        if m_value:
            values.append((m_value.group('name'), int(m_value.group('value'))))

        elif m_mapping:
            input = m_mapping.group('input')
            low_output = m_mapping.group('low_output')
            high_output = m_mapping.group('high_output')

            Bot.get(input).low_output = low_output
            Bot.get(input).high_output = high_output

for name, value in values:
    Bot.get(name).give(value)

if args.debug:
    print('\n-- final state --')
    for bot in Bot.all():
        print(bot)

print('\n-- part 1 --')
for bot in Bot.all():
    if set(args.targets) in bot.compared:
        print(bot, 'compared', args.targets)

print('\n-- part 2 --')
print('output0 * output1 * output2 = {}'.format(
    list(Bot.get('output 0').values)[0]
    * list(Bot.get('output 1').values)[0]
    * list(Bot.get('output 2').values)[0]
))
