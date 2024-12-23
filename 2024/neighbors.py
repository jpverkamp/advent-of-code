import collections, sys

neighbors = collections.Counter()

for line in sys.stdin:
    (a, b) = line.strip().split("-")
    neighbors[a] += 1
    neighbors[b] += 1

by_count = collections.defaultdict(set)
for node, count in neighbors.items():
    by_count[count].add(node)

for count in sorted(by_count.keys(), reverse=True):
    nodes = ",".join(sorted(node for node in by_count[count]))
    print(f"{count}: {nodes}")
