from pprint import pprint
from collections import deque
from copy import deepcopy


def remove_rgb(size: int, board: list, rg_same: bool = False):
    queue = deque()
    colors = dict()

    if rg_same:
        for y in range(size):
            for x in range(size):
                board[y][x][1] = False
                if board[y][x][0] == 'G':
                    board[y][x][0] = 'R'

    for y in range(size):
        for x in range(size):
            if board[y][x][1]:
                continue

            queue.append((x, y))

            color = board[y][x][0]

            if color in colors:
                colors[color] += 1
            else:
                colors[color] = 1

            while queue:
                x, y = queue.popleft()
                if board[y][x][1]:
                    continue
                board[y][x][1] = True

                if x > 0 and board[y][x-1][0] == color:
                    queue.append((x-1, y))
                if x < size-1 and board[y][x+1][0] == color:
                    queue.append((x+1, y))
                if y > 0 and board[y-1][x][0] == color:
                    queue.append((x, y-1))
                if y < size-1 and board[y+1][x][0] == color:
                    queue.append((x, y+1))

    return sum(colors.values())


def main():
    size = int(input())
    original_board = [list(map(lambda x: [x, False], input())) for _ in range(size)]

    print(remove_rgb(size, deepcopy(original_board)), end=' ')
    print()
    print(remove_rgb(size, original_board, True))


if __name__ == '__main__':
    main()
