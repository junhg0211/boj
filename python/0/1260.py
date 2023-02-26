from collections import deque
from sys import stdin

input = stdin.readline


def dfs(nodes, node, been=None):
    if been is None:
        been = set()

    been.add(node)
    print(node, end=' ')

    for node in nodes[node]:
        if node not in been:
            dfs(nodes, node, been)


def bfs(nodes, node, been=None):
    if been is None:
        been = set()

    queue = deque([node])

    while queue:
        node = queue.popleft()
        if node not in been:
            been.add(node)
            print(node, end=' ')
            queue.extend(nodes[node])


def main():
    _, edge_count, start = map(int, input().split())
    nodes = {start: list()}

    for _ in range(edge_count):
        a, b = map(int, input().split())

        if a in nodes:
            nodes[a].append(b)
        else:
            nodes[a] = [b]

        if b in nodes:
            nodes[b].append(a)
        else:
            nodes[b] = [a]

    for value in nodes.values():
        value.sort()

    dfs(nodes, start)
    print()

    bfs(nodes, start)
    print()


if __name__ == '__main__':
    main()
