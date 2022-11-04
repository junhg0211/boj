from copy import copy
from sys import stdin


def main():
    height, width = map(int, stdin.readline().rstrip().split())
    board = [list(map(int, stdin.readline().rstrip())) for _ in range(height)]

    queue = [(0, 0, 1)]
    history = 1

    while True:
        x, y, history = queue.pop(0)
        board[y][x] = 2

        if x > 0 and board[y][x-1] == 1:
            queue.append((x-1, y, history+1))
        if x < width - 1 and board[y][x+1] == 1:
            queue.append((x+1, y, history+1))
        if y > 0 and board[y-1][x] == 1:
            queue.append((x, y-1, history+1))
        if y < height - 1 and board[y+1][x] == 1:
            queue.append((x, y+1, history+1))

        if x == width - 1 and y == height - 1:
            break

    print(history)


if __name__ == '__main__':
    main()
