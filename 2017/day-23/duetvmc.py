#!/usr/bin/env python3

import functools
import re

import sys; sys.path.insert(0, '..'); import lib

code = '\n'.join(lib.input())
compilation_steps = []

def compile_step(f):
    functools.wraps(f)
    def new_f(code):
        lib.log(f'Applying {f.__name__} to:\n{code}\n')
        return f(code)

    compilation_steps.append(new_f)
    return f

@compile_step
def rewrite_simple_binops(code):
    '''Rewrite simple binary ops that have a direct python equivalent.'''

    for name, symbol in [('set', ''), ('sub', '-'), ('mul', '*')]:
        code = re.sub(
            r'{} ([a-h]) ([a-h]|-?\d+)'.format(name),
            r'\1 {}= \2'.format(symbol),
            code
        )
    return code

@compile_step
def rewrite_jumps_as_absolute(code):
    '''Rewrite jumps in an absolute form with a label at the destination.'''

    labels = {}
    lines = code.split('\n')

    for index, line in enumerate(lines):
        m = re.match(r'jnz ([a-h]|-?\d+) (-?\d+)', line)
        if m:
            register, offset = m.groups()
            offset = int(offset)

            if 0 <= index + offset < len(lines):
                label = 'L{}'.format(len(labels))
                lib.log(f'Detected new jump {label} for {register} with offset {offset}')

                labels[label] = index + offset
                lines[index] = 'jnz {} {}'.format(register, label)
            else:
                lib.log(f'Detected jump out of bounds (recompiling as a halt)')
                lines[index] = 'sys.exit(0)'


    for label, index in labels.items():
        lines[index] = '# {}\n{}'.format(label, lines[index])

    return '\n'.join(lines)

# HACK, reorder labels
@compile_step
def hack(code):
    return re.sub('# L7\n# L1', '# L1\n# L7', code)

@compile_step
def rewrite_jumps(code):
    '''Using the previous step, rewrite jumps.'''

    def rewrite_simple_if(code):
        '''Rewrite non-nested foward jumps as simple ifs.'''

        def make_if(m):
            indentation, register, label, body = m.groups()
            lib.log(f'Converting jump {label} ({register} = 0) to an if block, body:\n{body}\n')

            return '{}if {} == 0:\n{}'.format(
                indentation,
                register,
                '\n'.join('    ' + line for line in body.split('\n')),
            )

        return re.sub(
            r'(\s*)jnz ([a-h]|-?\d+) (L\d+)\n((?!jnz|# L).*)\n\1# \3',
            make_if,
            code,
            flags = re.DOTALL
        )

    def rewrite_if_not(code):
        '''Rewrite an overlapping pair of forward jumps as if/else.'''

        def make_if_not(m):
            indentation, register, label_1, label_2, body = m.groups()
            lib.log(f'Converting complex jump {label_1}/{label_2} to an if block, body:\n{body}\n')

            return '''\
{indentation}if {register} != 0:
{body}\
'''.format(
    indentation = indentation,
    register = register,
    body = '\n'.join('    ' + line for line in body.split('\n')),
)

        return re.sub(
            r'(\s*)jnz ([a-h]) (L\d+)\n\1jnz (?!0)-?\d+ (L\d+)\n\1# \3\n((?!jnz|# L).*)\n\1# \4',
            make_if_not,
            code,
            flags = re.DOTALL
        )

    def rewrite_simple_while(code):
        '''Rewrite non-nested backward jumps as while loops with a flag.'''

        def make_while(m):
            indentation, label, body, register = m.groups()
            lib.log(f'Converting jump {label} ({register} = 0) to a while block, body:\n{body}\n')

#{indentation}flag_{label} = True
#{indentation}while flag_{label} or {register} != 0:
#{indentation}    flag_{label} = False
#{body}\

            return '''\
{indentation}# {label}
{indentation}while True:
{body}
{indentation}    if {register} == 0: break\
'''.format(
    indentation = indentation,
    register = register,
    label = label,
    body = '\n'.join('    ' + line for line in body.split('\n')),
)

        return re.sub(
            r'(\s*)# (L\d+)\n((?!jnz|# L).*)\n\1jnz ([a-h]|-?\d+) \2',
            make_while,
            code,
            flags = re.DOTALL
        )

    # Keep running these functions until we reach a stable state
    functions = [
        rewrite_simple_if,
        rewrite_if_not,
        rewrite_simple_while,
    ]

    while True:
        new_code = code
        for f in functions:
            new_code = f(new_code)

        if code == new_code:
            break
        else:
            code = new_code

    return code

if lib.part(1):
    @compile_step
    def add_debug_statements(code):
        code = re.sub(r'([a-h]) \*=', r'mul_count += 1; \1 *=', code)
        code = re.sub(r'sys.exit', 'print(mul_count); sys.exit', code)
        code = 'mul_count = 0\n' + code + '\nprint(mul_count)'
        return code

    @compile_step
    def add_initial_registers(code):
        return 'a = b = c = d = e = f = g = h = 0\n' + code

if lib.part(2):
    @compile_step
    def add_initial_registers(code):
        return 'a = 1\nb = c = d = e = f = g = h = 0\n' + code

    # HACK
    @compile_step
    def replace_with_composite_counter(code):
        def comment_out(m):
            return '\n'.join('# ' + line for line in m.group(1).split('\n')) + '\n'

        code = re.sub(r'(# L7\n.*)', comment_out, code, flags = re.DOTALL)
        code += '''
# Turns out the code is checking for composite numbers... very inefficiently
# a != 0 sets up the range on b and c (note the off by one for c...)
# The L7 loop is looping from b to c by 17 (the b -= -17 at the end)
# The L4 loop is looping from 2 to g, setting f = 0 if g is divisible by the given numbers
# - NOTE: The original loop doesn't bail out early, which helps speed up a fair bit
# The L3 loop is doing the trial division ()

for g in range(b, c + 1, 17):
    for e in range(2, g):
        if g % e == 0:
            h += 1
            break
print(h)
'''
        return code

@compile_step
def add_imports(code):
    return 'import sys\n' + code

for step in compilation_steps:
    code = step(code)

print(f'# Final code:\n{code}')
