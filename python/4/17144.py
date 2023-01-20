from sys import stdin

input = stdin.readline


def copy(from_, to, width, height):
    for i in range(height):
        for j in range(width):
            to[i][j] = from_[i][j]


def main():
    height, width, eta = map(int, input().split())
    board = [list(map(int, input().split())) for _ in range(height)]
    blank = [[0 for _ in range(width)] for _ in range(height)]
    next_board = [[0 for _ in range(width)] for _ in range(height)]

    # find where is purifier
    purifier_y = None
    for i in range(height):
        if board[i][0] == -1:
            purifier_y = i
            break

    blank[purifier_y][0] = -1
    blank[purifier_y+1][0] = -1

    while eta:
        copy(blank, next_board, width, height)

        # diffusion
        for i in range(height):
            for j in range(width):
                if not board[i][j]:
                    continue
                if board[i][j] == -1:
                    continue

                direction_count = 0
                amount = board[i][j] // 5

                if i > 0 and board[i-1][j] != -1:
                    next_board[i-1][j] += amount
                    direction_count += 1
                if j > 0 and board[i][j-1] != -1:
                    next_board[i][j-1] += amount
                    direction_count += 1
                if i < height-1 and board[i+1][j] != -1:
                    next_board[i+1][j] += amount
                    direction_count += 1
                if j < width-1 and board[i][j+1] != -1:
                    next_board[i][j+1] += amount
                    direction_count += 1

                next_board[i][j] += board[i][j] - amount * direction_count

        copy(next_board, board, width, height)

        # purify
        for i in range(purifier_y-1, -1, -1):
            if i == purifier_y - 1:
                board[i][0] = 0
                continue
            board[i+1][0] = board[i][0]
        for i in range(purifier_y+2, height):
            if i == purifier_y + 2:
                board[i][0] = 0
                continue
            board[i-1][0] = board[i][0]

        for i in range(1, width):
            board[0][i-1] = board[0][i]
            board[height-1][i-1] = board[height-1][i]

        for i in range(1, purifier_y+1):
            board[i-1][width-1] = board[i][width-1]
        for i in range(height-2, purifier_y, -1):
            board[i+1][width-1] = board[i][width-1]

        for i in range(width-2, -1, -1):
            value = board[purifier_y][i]
            if value == -1:
                value = 0
            board[purifier_y][i+1] = value

            value = board[purifier_y+1][i]
            if value == -1:
                value = 0
            board[purifier_y+1][i+1] = value

        eta -= 1

    result = 2
    for row in board:
        result += sum(row)

    print(result)


if __name__ == '__main__':
    main()
