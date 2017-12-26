#!/usr/bin/env python3

import collections
import queue
import threading
import time

import sys; sys.path.insert(0, '..'); import lib

class VM(object):
    vms = []

    def __init__(self):
        '''Initialize a VM.'''

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

    def __call__(self, code, daemon = False):
        '''Run the given code with the given VM.'''

        if daemon:
            t = threading.Thread(target = self, args = [code])
            t.daemon = True
            t.start()
            return t

        try:
            self.state = 'running'

            while 0 <= self.pc < len(code):
                cmd, *args = code[self.pc]
                lib.log('{}, {}: {}({}); {}', self.id, self.pc, cmd, args, dict(self.registers))
                getattr(self, cmd)(*args)
                self.pc += 1

        except StopIteration:
            pass

        lib.log('{}, {}: EXITING; {}', self.id, self.pc, dict(self.registers))
        self.state = 'exited'

        return self.output

@VM.register
def snd(vm, x):
    vm.output.append(vm.value(x))

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
            print(f'Recovered {vm.output[-1]}')
            raise StopIteration

    vm = VM()
    vm(code)

elif lib.part(2):
    vm0 = VM()
    vm1 = VM()

    @VM.register
    def snd(vm, x):
        if not hasattr(vm, 'send_count'): vm.send_count = 0
        vm.send_count += 1

        index = VM.vms.index(vm)
        VM.vms[(index + 1) % len(VM.vms)].messages.put(vm.value(x))

    @VM.register
    def rcv(vm, x):
        vm.state = 'waiting'

        value = vm.messages.get()
        if value == StopIteration:
            raise StopIteration
        else:
            vm.registers[x] = value

        vm.state = 'running'

    vm0(code, daemon = True)
    vm1(code, daemon = True)

    while True:
        if vm0.state == vm1.state == 'waiting':
            time.sleep(1)
            if vm0.state == vm1.state == 'waiting':
                lib.log('DEADLOCK DETECTED')
                vm0.messages.put(StopIteration)
                vm1.messages.put(StopIteration)
                break

    while True:
        if vm0.state == vm1.state == 'exited':
            break

    print(vm1.send_count)
