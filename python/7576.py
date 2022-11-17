from sys import stdin

input = stdin.readline


def main():
    width, height = map(int, input().split())

    ripens = list()

    board = list()
    for i in range(height):
        line = list(map(int, input().split()))
        for j in range(width):
            if line[j] == 1:
                ripens.append((j, i))
        board.append(line)

    day_starter = None
    day = 0

    while ripens:
        ripen = ripens.pop(0)
        x, y = ripen
        board[y][x] = 1

        if ripen == day_starter:
            day_starter = None
            day += 1

        if x > 0 and board[y][x-1] == 0:
            ripens.append((x-1, y))
            if day_starter is None:
                day_starter = ripens[-1]
        if x < width - 1 and board[y][x+1] == 0:
            ripens.append((x+1, y))
            if day_starter is None:
                day_starter = ripens[-1]
        if y > 0 and board[y-1][x] == 0:
            ripens.append((x, y-1))
            if day_starter is None:
                day_starter = ripens[-1]
        if y < height - 1 and board[y+1][x] == 0:
            ripens.append((x, y+1))
            if day_starter is None:
                day_starter = ripens[-1]

    for i in range(height):
        for j in range(width):
            if board[i][j] == 0:
                print('-1')
                return

    print(day)


if __name__ == '__main__':
    main()
