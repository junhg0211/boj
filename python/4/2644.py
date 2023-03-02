from collections import deque
from sys import stdin

input = stdin.readline


def bfs(start, end, connections):
    been = {start}
    nows = deque([(start, 0)])

    while nows:
        now, chon = nows.popleft()

        if now == end:
            return chon

        for connection in connections.get(now, ()):
            if connection not in been:
                nows.append((connection, chon+1))
                been.add(connection)


def main():
    nodes = int(input())
    start, end = map(int, input().split())

    connections = dict()
    for _ in range(int(input())):
        a, b = map(int, input().split())

        if a not in connections:
            connections[a] = [b]
        else:
            connections[a].append(b)

        if b not in connections:
            connections[b] = [a]
        else:
            connections[b].append(a)

    if (chon := bfs(start, end, connections)) is None:
        print('-1')
    else:
        print(chon)


if __name__ == '__main__':
    main()
