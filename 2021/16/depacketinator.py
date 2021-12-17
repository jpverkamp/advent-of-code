import logging
from types import FunctionType
import typer
import operator

from collections import *
from itertools import islice
from dataclasses import dataclass
from typing import *

app = typer.Typer()


def disableable_cache(x): return x


class BitStreamException(Exception):
    pass


class PacketParseException(Exception):
    pass


@dataclass
class BitStream:
    index: int
    data: str

    @staticmethod
    def from_hex(hex_data: str) -> 'BitStream':
        return BitStream(0, ''.join(
            '{:04b}'.format(int(c, 16)) for c in hex_data
        ))

    def read_bits(self, n: int) -> str:
        '''Read n bits from the current position in the bitsting as a string of 0/1'''

        value = self.data[self.index:self.index+n]
        self.index += n

        if self.index > len(self.data):
            raise BitStreamException('Attempted to read past the end of bitstream')

        return value

    def read_int(self, n: int) -> int:
        '''Read n bits from the current position and convert to an integer'''

        return int(self.read_bits(n), 2)

    def read_bool(self) -> bool:
        '''Read the next bit from the current position as True/False'''

        return self.read_bits(1) == '1'

    def __str__(self):
        if self.index + 8 < len(self.data):
            return f'<{self.index}/{len(self.data)}, {self.data[self.index:self.index+8]}...>'
        else:
            return f'<{self.index}/{len(self.data)}, {self.data[self.index:]}>'


@dataclass
class Packet:
    version: int
    type_id: int

    value: int
    children: List['Packet']

    length: int

    @staticmethod
    def from_hex(hex: str) -> 'Packet':
        return Packet.from_bitstream(BitStream.from_hex(hex))

    @staticmethod
    def from_bitstream(bits: BitStream, _depth: int = 0) -> 'Packet':
        logging.info(f'{" " * _depth}Parsing new packet at {bits}')

        version = bits.read_int(3)
        type_id = bits.read_int(3)
        length = 6

        value = 0
        children = []

        logging.info(f'{" " * _depth} - {version=}, {type_id=}')

        # Literal values
        if type_id == 4:
            logging.info(f'{" " * _depth} - Mode=literal')

            keep_reading = True
            while keep_reading:
                keep_reading = bits.read_bool()
                byte = bits.read_int(4)
                logging.info(f'{" " * _depth} - Read byte {byte}, will continue: {keep_reading}')

                value = value * 16 + byte
                length += 5

        # Any other operator value
        else:
            # The next bit is the length_type_id
            # If it's set, read the number of bits in subpackets
            length += 1
            if bits.read_bool():
                length += 11
                number_of_children = bits.read_int(11)
                logging.info(f'{" " * _depth} - Mode=operator, length_type=1 ({number_of_children} children)')

                for _ in range(number_of_children):
                    child = Packet.from_bitstream(bits, _depth + 1)
                    children.append(child)

                    length += child.length

            # If it's not, read the number of subpackets
            else:
                length += 15
                body_length = bits.read_int(15)
                logging.info(f'{" " * _depth} - Mode=operator, length_type=0 ({body_length} bits)')
                logging.info(f'{" " * _depth} - {len(bits.data)-bits.index} of {len(bits.data)} remaining')

                while body_length:
                    child = Packet.from_bitstream(bits, _depth + 1)
                    children.append(child)

                    body_length -= child.length
                    length += child.length

                    logging.info(f'{" " * _depth} - New child used {child.length} bits, {body_length} remaining')

                    if body_length < 0:
                        raise PacketParseException('Could not parse packet, too many bits used by children')

        p = Packet(version, type_id, value, children, length)
        logging.info(f'{" " * _depth} \ Packet parsed: {p}')
        return p


@app.command()
def part1(file: typer.FileText):

    def sum_versions(p: Packet) -> int:
        return p.version + sum(sum_versions(child) for child in p.children)

    for line in file:
        line = line.strip()
        if not line:
            continue

        p = Packet.from_hex(line)
        logging.info(p)
        print(f'{sum_versions(p):-16} {line}')


@app.command()
def part2(file: typer.FileText):

    def prod(ls):
        result = 1
        for el in ls:
            result *= el
        return result

    operators: Mapping[int, Callable[[List[int]], int]] = {
        0: sum,
        1: prod,
        2: min,
        3: max,
        5: lambda ab: 1 if ab[0] > ab[1] else 0,
        6: lambda ab: 1 if ab[0] < ab[1] else 0,
        7: lambda ab: 1 if ab[0] == ab[1] else 0,
    }

    def a_better_eval(p: Packet) -> int:
        # Literal values first
        if p.type_id == 4:
            result = p.value

        # Otherwise, parse children
        else:
            values = [a_better_eval(child) for child in p.children]
            f = operators[p.type_id]
            result = f(values)

        logging.info(f'a_better_eval({p}) = {result}')
        return result

    for line in file:
        line = line.strip()
        if not line:
            continue

        p = Packet.from_hex(line)
        print(f'{a_better_eval(p):-16} {line}')


@app.callback()
def enableFlags(cache: bool = False, debug: bool = False):
    if debug:
        import coloredlogs  # type: ignore
        coloredlogs.install(level=logging.INFO)

    if cache:
        import functools
        global disableable_cache
        disableable_cache = functools.cache


if __name__ == '__main__':
    app()
