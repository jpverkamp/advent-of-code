#!/usr/bin/env python3

import math
import pprint

import sys; sys.path.insert(0, '..'); import lib
lib.add_argument('--state', default = '.#./..#/###', help = 'The initial state to expand')
lib.add_argument('--iterations', type = int, required = True, help = 'The number of times to expand')
lib.add_argument('--render', help = 'If specified, render the final image to the given filename')
lib.add_argument('--render-steps', help = 'If specified, render each final image to the given filename with the step included')

rules = dict(
    line.replace('/', '').split(' => ')
    for line in lib.input(include_comments = True)
)

lib.log('{} rules loaded:\n{}', len(rules), pprint.pformat(rules))

def render_block(data):
    '''Render a square block to a string.'''

    size = int(math.sqrt(len(data)))

    return '\n'.join(
        ''.join(data[y * size + x] for x in range(size))
        for y in range(size)
    )

def render_image(data, filename):
    '''Render a block as an image (assumes # is black and . is white).'''

    size = int(math.sqrt(len(data)))

    def generate_pixel(x, y):
        g = 0 if data[y * size + x] == '#' else 255
        return (g, g, g)

    lib.generate_image(size, size, generate_pixel).save(filename)

def blocks(data):
    '''
    Convert data into blocks.

    If data has an even number of elements, return 2x2 blocks.
    If it has an odd number, return 3x3 blocks.

    Assume data is a perfect square.
    '''

    lib.log(f'Generating blocks from {data}')

    if len(data) % 2 == 0:
        block_size = 2
    else:
        block_size = 3

    data_row_width = int(math.sqrt(len(data)))
    grid_size = data_row_width // block_size

    lib.log(f'Data length: {len(data)}, block size: {block_size}, row width: {data_row_width}, grid_size: {grid_size}')

    for grid_y in range(grid_size):
        for grid_x in range(grid_size):
            yield ''.join(
                data[data_row_width * block_size * grid_y + data_row_width * block_y + block_size * grid_x + block_x]
                for block_y in range(block_size)
                for block_x in range(block_size)
            )

def deblock(data):
    '''
    Inverse of the above, turn a list of blocks back into data.
    '''

    lib.log(f'Deblocking {data}')

    block_size = int(math.sqrt(len(data[0])))
    grid_size = int(math.sqrt(len(data)))

    lib.log(f'grid sized {grid_size}, block sized {block_size}')

    return ''.join(
        data[grid_y * grid_size + grid_x][block_y * block_size + block_x]
        for grid_y in range(grid_size)
        for block_y in range(block_size)
        for grid_x in range(grid_size)
        for block_x in range(block_size)
    )

def rotations(block):
    '''Yield the 8 rotations/flips of a block.'''

    size = int(math.sqrt(len(block)))

    for i in range(2):
        for j in range(4):
            yield block

            # Rotate 90 degrees clockwise
            block = ''.join(
                block[(size - x - 1) * size + y]
                for y in range(size)
                for x in range(size)
            )

        # Flip vertically
        block = ''.join(
            block[(size - y - 1) * size + x]
            for y in range(size)
            for x in range(size)
        )

def make_grid(size):
    '''Test function: create a grid of unique characters.'''

    return ''.join(
        chr(0x4E00 + y * size + x)
        for y in range(size)
        for x in range(size)
    )

def expand_block(block):
    '''Expand a single block.'''

    lib.log(f'Expanding {block}')

    for rotation in rotations(block):
        lib.log(f'Trying {rotation}')
        if rotation in rules:
            lib.log(f'Found a matching rule: {rules[rotation]}')
            return rules[rotation]

    raise Exception(f'Unable to expand {block}')

def expand(data):
    '''Expand a block.'''

    lib.log(f'Expanding {data}')
    lib.log(f'Blocks are {list(blocks(data))}')

    return deblock([
        expand_block(block)
        for block in blocks(data)
    ])

data = lib.param('state').replace('/', '')
for round in range(lib.param('iterations')):
    lib.log(f'Expanding:\n{render_block(data)}\n')
    data = expand(data)

    if lib.param('render_steps'):
        filename, ext = lib.param('render_steps').rsplit('.', 1)
        render_image(data, f'{filename}-{round:04d}.{ext}')

print(render_block(data).count('#'))

if lib.param('render'):
    render_image(data, lib.param('render'))
