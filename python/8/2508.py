from sys import stdin

input = stdin.readline


def tick():
    input()
    height, width = map(int, input().split())
    board = tuple(input() for _ in range(height))

    count = 0
    for i in range(height):
        for j in range(width):
            if board[i][j] != 'o':
                continue

            if 0 < i < height-1 \
                    and board[i-1][j] == 'v' \
                    and board[i+1][j] == '^':
                count += 1
                continue

            if 0 < j < width-1 \
                    and board[i][j-1] == '>' \
                    and board[i][j+1] == '<':
                count += 1

    print(count)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
