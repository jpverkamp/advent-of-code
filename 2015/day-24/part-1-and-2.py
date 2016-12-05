#!/usr/bin/env python3

import itertools
import sys

def subsets_summing_to(target, items, cache = {}):
    if target == 0:
        yield set()
    else:
        for i, item in enumerate(items):
            if item <= target:
                for recur in subsets_summing_to(target - item, items - {item}):
                    yield {item} | recur

def subset_sum_of_n(target, items, count):
    if target == 0 and count == 0:
        yield set()
    elif count == 0:
        return
    else:
        for i, item in enumerate(sorted(items)):
            if item <= target:
                for recur in subset_sum_of_n(target - item, items - {item}, count - 1):
                    yield {item} | recur

def calculate_quantum_entanglement(group):
    product = 1
    for item in group:
        product *= item
    return product

def split_into(packages, n_groups):
    weight_per_section = sum(packages) / int(sys.argv[1])

    for n in range(1, len(packages)):
        for group in subset_sum_of_n(weight_per_section, packages, n):
            return (len(group), calculate_quantum_entanglement(group), group)

if __name__ == '__main__':
    packages = {int(line.strip()) for line in sys.stdin}
    n_groups = int(sys.argv[1])

    print(split_into(packages, n_groups))
