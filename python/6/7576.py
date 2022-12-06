from collections import deque
from sys import stdin

input = stdin.readline


def main():
    width, height = map(int, input().split())

    ripens = deque()

    board = list()
    for i in range(height):
        line = list(map(int, input().split()))
        for j, data in enumerate(line):
            if data == 1:
                ripens.append((j, i))
        board.append(line)

    day_starter = None
    day = 0

    while ripens:
        ripen = ripens.popleft()
        x, y = ripen

        if ripen == day_starter:
            day_starter = None
            day += 1

        if x > 0 and board[y][x-1] == 0:
            ripens.append((x-1, y))
            board[y][x-1] = 1
            if day_starter is None:
                day_starter = ripens[-1]
        if x < width - 1 and board[y][x+1] == 0:
            ripens.append((x+1, y))
            board[y][x+1] = 1
            if day_starter is None:
                day_starter = ripens[-1]
        if y > 0 and board[y-1][x] == 0:
            ripens.append((x, y-1))
            board[y-1][x] = 1
            if day_starter is None:
                day_starter = ripens[-1]
        if y < height - 1 and board[y+1][x] == 0:
            ripens.append((x, y+1))
            board[y+1][x] = 1
            if day_starter is None:
                day_starter = ripens[-1]

    for line in board:
        if 0 in line:
            print('-1')
            return

    print(day)


if __name__ == '__main__':
    main()
