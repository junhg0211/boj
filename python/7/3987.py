from math import inf
from sys import stdin

input = stdin.readline

width, height = 0, 0


def go(x: int, y: int, direction: int, board: list):
    been = set()
    path = 0

    while True:
        been.add((x, y, direction))
        now = board[y-1][x-1]

        if now == 'C':
            return path

        path += 1

        if direction == 0:
            if now == '/':
                direction = 1
                x += 1
            elif now == '\\':
                direction = 3
                x -= 1
            else:
                y -= 1
        elif direction == 1:  # RIGHT
            if now == '/':
                direction = 0
                y -= 1
            elif now == '\\':
                direction = 2
                y += 1
            else:
                x += 1
        elif direction == 2:  # DOWN
            if now == '/':
                direction = 3
                x -= 1
            elif now == '\\':
                direction = 1
                x += 1
            else:
                y += 1
        elif direction == 3:  # LEFT
            if now == '/':
                direction = 2
                y += 1
            elif now == '\\':
                direction = 0
                y -= 1
            else:
                x -= 1

        if x < 1 or x > width or y < 1 or y > height:
            return path
        if (x, y, direction) in been:
            return inf


def sort_key(x: tuple):
    return (x[0], -'URDL'.index(x[1]))


def main():
    global width, height

    height, width = map(int, input().split())
    board = [input() for _ in range(height)]
    y, x = map(int, input().split())

    up = go(x, y, 0, board)
    right = go(x, y, 1, board)
    down = go(x, y, 2, board)
    left = go(x, y, 3, board)

    result = max((up, 'U'), (right, 'R'), (down, 'D'), (left, 'L'), key=sort_key)

    print(result[1])
    print('Voyager' if result[0] == inf else result[0])


if __name__ == '__main__':
    main()
