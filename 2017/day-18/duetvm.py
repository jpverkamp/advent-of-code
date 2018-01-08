#!/usr/bin/env python3

import collections
import queue
import threading
import time

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--write-midi', help = 'Filename to write a MIDI file of the output to, if part(2): filename-0.ext and filename-1.ext will be used')
lib.add_argument('--clock', type = int, default = 10, help = 'How many commands to run per beat (for MIDI output)')

class VM(object):
    vms = []

    def __init__(self):
        '''Initialize a VM.'''

        self.tick = 0
        self.pc = 0
        self.registers = collections.defaultdict(lambda : 0)
        self.output = []
        self.messages = queue.Queue()

        self.id = len(VM.vms)
        self.registers['p'] = self.id
        VM.vms.append(self)

        self.state = 'ready'

    @staticmethod
    def register(f, name = None):
        setattr(VM, name or f.__name__, f)
        return f

    def value(self, key):
        '''If key is a number, return that number; if it's a register, return it's value.'''

        val = self.registers.get(key, key)
        try:
            return int(val)
        except:
            return val

    def write_midi(self, filename):
        '''Write all of the output of the program so far as a MIDI file.'''

        import math
        import midiutil

        if self.output:
            offset = self.output[0][0]
        else:
            offset = 0

        clock = lib.param('clock')

        midi = midiutil.MIDIFile(1)
        midi.addTempo(
            0,          # Track
            0,          # Start time
            120,        # Tempo (BPM)
        )

        for tick, frequency in self.output:
            # https://en.wikipedia.org/wiki/MIDI_tuning_standard#Frequency_values
            pitch = int(69 + 12 * math.log(frequency / 440))
            midi.addNote(
                0,      # Track
                0,      # Channel
                pitch,  # Pitch of the note (midi data values)
                (tick - offset) / clock, # Tick to add the note
                1,      # Duration (beats)
                100,    # Volume (0-127)
            )

        with open(filename, 'wb') as fout:
            midi.writeFile(fout)

    def __call__(self, code, daemon = False, generator = False):
        '''
        Run the given code with the given VM.

        If daemon is True, spawn a background thread to run the program in.
        If generator is True, return a generator that yields once per tick.
        '''

        if daemon and generator:
            raise Exception('Specify only one of daemon and generator')

        if daemon:
            t = threading.Thread(target = self, args = [code])
            t.daemon = True
            t.start()
            return t

        try:
            self.state = 'running'

            while 0 <= self.pc < len(code):
                self.tick += 1
                cmd, *args = code[self.pc]
                lib.log('{}: {}, {}: {}({}); {}', self.tick, self.id, self.pc, cmd, args, dict(self.registers))
                getattr(self, cmd)(*args)
                self.pc += 1

                if generator:
                    yield

        except StopIteration:
            pass

        lib.log('{}, {}: EXITING; {}', self.id, self.pc, dict(self.registers))
        self.state = 'exited'

        return self.output

@VM.register
def snd(vm, x):
    vm.output.append((vm.tick, vm.value(x)))

@VM.register
def set(vm, x, y):
    vm.registers[x] = vm.value(y)

@VM.register
def add(vm, x, y):
    vm.registers[x] += vm.value(y)

@VM.register
def mul(vm, x, y):
    vm.registers[x] *= vm.value(y)

@VM.register
def mod(vm, x, y):
    vm.registers[x] %= vm.value(y)

@VM.register
def jgz(vm, x, y):
    if vm.value(x) > 0:
        vm.pc += vm.value(y) - 1

code = [line.split() for line in lib.input()]

if lib.part(1):
    @VM.register
    def rcv(vm, x):
        if vm.value(x) != 0 and vm.output:
            print(f'Recovered {vm.output[-1][1]}')
            raise StopIteration

    vm = VM()
    for step in vm(code):
        pass

    if lib.param('write_midi'):
        vm.write_midi(lib.param('write_midi'))

elif lib.part(2):
    vm0 = VM()
    vm1 = VM()

    @VM.register
    def snd(vm, x):
        if not hasattr(vm, 'send_count'): vm.send_count = 0
        vm.send_count += 1

        index = VM.vms.index(vm)
        VM.vms[(index + 1) % len(VM.vms)].messages.put(vm.value(x))

        vm.output.append((vm.tick, vm.value(x)))

    @VM.register
    def rcv(vm, x):
        vm.state = 'waiting'

        try:
            value = vm.messages.get(block = False)
            if value == StopIteration:
                raise StopIteration
            else:
                vm.registers[x] = value

            vm.state = 'running'

        except queue.Empty:
            # Run the rcv command again next tick
            vm.pc -= 1

    generator0 = vm0(code, generator = True)
    generator1 = vm1(code, generator = True)

    while True:
        next(generator0)
        next(generator1)

        if vm0.state == 'waiting' and vm1.state == 'waiting':
            break

    print(vm1.send_count)

    if lib.param('write_midi'):
        filename, ext = lib.param('write_midi').rsplit('.', 1)
        vm0.write_midi(f'{filename}-0.{ext}')
        vm1.write_midi(f'{filename}-1.{ext}')
