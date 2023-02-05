from math import inf


def tick():
    height, width = map(int, input().split())
    board = [list(map(int, input().split())) for _ in range(height)]

    min_x = inf
    max_x = -inf
    min_y = inf
    max_y = -inf

    for i in range(height):
        for j in range(width):
            if board[i][j] != '#':
                continue

            min_x = min(min_x, j)
            max_x = max(max_x, j)
            min_y = min(min_x, i)
            max_y = max(max_x, i)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
