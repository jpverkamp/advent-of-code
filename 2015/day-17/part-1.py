#!/usr/bin/env python3

import sys

containers = list(map(int, sys.stdin.readlines()))
quantity = int(sys.argv[1])

def fills(quantity, containers):
    if quantity == 0:
        yield []
    else:
        for index, container in enumerate(containers):
            if container <= quantity:
                for sub_fill in fills(quantity - container, containers[index+1:]):
                    yield [container] + sub_fill

print(len(list(fills(quantity, containers))))
