from collections import deque
from sys import stdin

input = stdin.readline


def tick():
    w, h, n = map(int, input().split())
    board = dict()
    points = list()
    for _ in range(n):
        point = tuple(map(int, input().split()))
        board[point] = None
        points.append(point)

    worms = 0
    for point in points:
        if point not in board:
            continue

        worms += 1
        queue = deque([point])
        while queue:
            point = queue.popleft()
            x, y = point

            if point in board:
                del board[point]
            else:
                continue

            if x > 0 and (x-1, y) in board:
                queue.append((x-1, y))
            if x < w - 1 and (x+1, y) in board:
                queue.append((x+1, y))
            if y > 0 and (x, y-1) in board:
                queue.append((x, y-1))
            if y < h - 1 and (x, y+1) in board:
                queue.append((x, y+1))

    print(worms)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
