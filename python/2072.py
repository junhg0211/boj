from sys import stdin

input = stdin.readline

SIZE = 19


def get_lengths(board: list, x: int, y: int) -> tuple[int]:
    left_streak = True
    down_left_streak = True
    down_streak = True
    down_right_streak = True
    right_streak = True
    up_right_streak = True
    up_streak = True
    up_left_streak = True

    left, down_left, down, down_right, right, up_right, up, up_left = 0, 0, 0, 0, 0, 0, 0, 0

    for i in range(1, 5):
        left_available = x-i >= 0
        right_available = x+i < SIZE
        up_available = y-i >= 0
        down_available = y+i < SIZE

        if up_available:
            if left_available and up_left_streak and board[y-i][x-i] == board[y][x]:
                up_left += 1
            else:
                up_left_streak = False

            if up_streak and board[y-i][x] == board[y][x]:
                up += 1
            else:
                up_streak = False

            if right_available and up_right_streak and board[y-i][x+i] == board[y][x]:
                up_right += 1
            else:
                up_right_streak = False

        if left_available and left_streak and board[y][x-i] == board[y][x]:
            left += 1
        else:
            left_streak = False

        if right_available and right_streak and board[y][x+i] == board[y][x]:
            right += 1
        else:
            right_streak = False

        if down_available:
            if left_available and down_left_streak and board[y+i][x-i] == board[y][x]:
                down_left += 1
            else:
                down_left_streak = False

            if down_streak and board[y+i][x] == board[y][x]:
                down += 1
            else:
                down_streak = False

            if right_available and down_right_streak and board[y+i][x+i] == board[y][x]:
                down_right += 1
            else:
                down_right_streak = False

    return left + right, down_left + up_right, down + up, down_right + up_left


def main():
    count = int(input())

    board = [[0 for _ in range(SIZE)] for _ in range(SIZE)]
    been = False

    for i in range(count):
        x, y = map(lambda x: int(x) - 1, input().split())

        if been:
            continue

        board[y][x] = i%2 + 1

        lengths = get_lengths(board, x, y)
        if 4 in lengths:
            print(i+1)
            been = True

    if not been:
        print('-1')


if __name__ == '__main__':
    main()
