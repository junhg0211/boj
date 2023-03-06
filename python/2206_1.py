from collections import deque
from copy import copy


def main():
    height, width = map(int, input().split())
    board = [list(map(int, input().rstrip())) for _ in range(height)]

    queue = deque(((0, 0, 1, True, set()),))  # x, y, path, skill_available, been

    while queue:
        x, y, path, skill, been = queue.popleft()
        print(x, y, path, skill)
        been.add((x, y))

        if x == width-1 and y == height-1:
            print(path)
            return

        if x > 0 and (x-1, y) not in been:
            if board[y][x-1] == 0:
                queue.appendleft((x-1, y, path+1, skill, copy(been)))
            elif skill and board[y][x-1] == 1:
                queue.append((x-1, y, path+1, False, copy(been)))

        if y > 0 and (x, y-1) not in been:
            if board[y-1][x] == 0:
                queue.appendleft((x, y-1, path+1, skill, copy(been)))
            elif skill and board[y-1][x] == 1:
                queue.append((x, y-1, path+1, False, copy(been)))

        if x < width-1 and (x+1, y) not in been:
            if board[y][x+1] == 0:
                queue.appendleft((x+1, y, path+1, skill, copy(been)))
            elif skill and board[y][x+1] == 1:
                queue.append((x+1, y, path+1, False, copy(been)))

        if y < height-1 and (x, y+1) not in been:
            if board[y+1][x] == 0:
                queue.appendleft((x, y+1, path+1, skill, copy(been)))
            elif skill and board[y+1][x] == 1:
                queue.append((x, y+1, path+1, False, copy(been)))

    print('-1')


if __name__ == '__main__':
    main()
