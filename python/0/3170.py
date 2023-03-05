from collections import deque
from math import hypot
from sys import stdin

input = stdin.readline


def is_containing(cx, cy, cr, x, y):
    dx = x - cx
    dy = y - cy
    return hypot(dx, dy) <= cr


def main():
    width, height = map(int, input().split())
    cranes = [tuple(map(int, input().split())) for _ in range(int(input()))]
    count = int(input())
    equipments = [tuple(map(int, input().split())) for _ in range(count)]

    connections = dict()
    contains = dict()
    queue = deque()
    for i, (x1, y1, r1) in enumerate(cranes):
        # evaluate connections between cranes
        for j, (x2, y2, r2) in enumerate(cranes):
            if i == j:
                continue

            dx = x2 - x1
            dy = y2 - y1

            distance = hypot(dx, dy)
            if distance <= r1 + r2:
                if i in connections:
                    connections[i].add(j)
                else:
                    connections[i] = {j}
                if j in connections:
                    connections[j].add(i)
                else:
                    connections[j] = {i}

        contains[i] = set()
        # evaluate if a destination is in area of a crane
        for j, (x, y) in enumerate(equipments):
            if is_containing(x1, y1, r1, x, y):
                contains[i].add(j)

        # evaluate if a crane contains start point
        if is_containing(x1, y1, r1, width//2, 0):
            queue.append(i)

    been = set()
    availables = set()
    while queue:
        ci = queue.popleft()
        been.add(ci)

        availables |= contains[ci]

        for connection in connections.get(ci, ()):
            if connection not in been:
                queue.append(connection)

    font = ['NE', 'DA']
    print('\n'.join(font[i in availables] for i in range(count)))


if __name__ == '__main__':
    main()
