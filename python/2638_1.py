from collections import deque
from sys import stdin

input = stdin.readline


def fill_outside(board: list, width: int, height: int, x: int = 0, y: int = 0):
    x_queue = deque([x])
    y_queue = deque([y])

    while x_queue:
        x = x_queue.popleft()
        y = y_queue.popleft()

        board[y][x] = 2
        if x > 0 and board[y][x-1] == 0:
            x_queue.append(x-1)
            y_queue.append(y)
        if x < width - 1 and board[y][x+1] == 0:
            x_queue.append(x+1)
            y_queue.append(y)
        if y > 0 and board[y-1][x] == 0:
            x_queue.append(x)
            y_queue.append(y-1)
        if y < height - 1 and board[y+1][x] == 0:
            x_queue.append(x)
            y_queue.append(y+1)


def main():
    height, width = map(int, input().split())

    board = [list(map(int, input().split())) for _ in range(height)]

    fill_outside(board, width, height)

    turn = 0
    while True:
        aigoes = list()
        for i in range(height):
            for j in range(width):
                if board[i][j] != 1:
                    continue

                blank = 0
                if i > 0 and board[i-1][j] == 2:
                    blank += 1
                if i < height-1 and board[i+1][j] == 2:
                    blank += 1
                if j > 0 and board[i][j-1] == 2:
                    blank += 1
                if j < width-1 and board[i][j+1] == 2:
                    blank += 1

                if blank >= 2:
                    aigoes.append((j, i))

        if not aigoes:
            break

        while aigoes:
            x, y = aigoes.pop()
            board[y][x] = 0
            fill_outside(board, width, height, x, y)

        turn += 1

    print(turn)


if __name__ == '__main__':
    main()
