
from collections import deque
from copy import deepcopy
from sys import stdin

input = stdin.readline


def main():
    height, width = map(int, input().split())
    original_board = [list(map(int, input().rstrip())) for _ in range(height)]

    states = deque([(deepcopy(original_board), 0, 0, 1, 1)])
    # board, x, y, path_length, break_left

    while states:
        board, x, y, path, break_left = states.popleft()
        board[y][x] = 2

        if x == width-1 and y == height-1:
            print(path)
            return

        if x > 0:
            if board[y][x-1] == 0:
                next_board = deepcopy(board)
                states.append((next_board, x-1, y, path+1, break_left))
            elif break_left and board[y][x-1] == 1:
                next_board = deepcopy(board)
                states.append((next_board, x-1, y, path+1, 0))
        if y > 0:
            if board[y-1][x] == 0:
                next_board = deepcopy(board)
                states.append((next_board, x, y-1, path+1, break_left))
            elif break_left and board[y-1][x] == 1:
                next_board = deepcopy(board)
                states.append((next_board, x, y-1, path+1, 0))
        if x < width-1:
            if board[y][x+1] == 0:
                next_board = deepcopy(board)
                states.append((next_board, x+1, y, path+1, break_left))
            elif break_left and board[y][x+1] == 1:
                next_board = deepcopy(board)
                states.append((next_board, x+1, y, path+1, 0))
        if y < height-1:
            if board[y+1][x] == 0:
                next_board = deepcopy(board)
                states.append((next_board, x, y+1, path+1, break_left))
            elif break_left and board[y+1][x] == 1:
                next_board = deepcopy(board)
                states.append((next_board, x, y+1, path+1, 0))

    print('-1')


if __name__ == '__main__':
    main()
