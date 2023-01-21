from sys import stdin

input = stdin.readline


def main():
    height, width = map(int, input().split())
    board = [list(map(int, input().split())) for _ in range(height)]

    turn = 0
    while True:
        aigo = 0
        for i in range(height):
            for j in range(width):
                if board[i][j] != 1:
                    continue

                blank = 0
                if i > 0 and not board[i-1][j]:
                    blank += 1
                if i < height-1 and not board[i+1][j]:
                    blank += 1
                if j > 0 and not board[i][j-1]:
                    blank += 1
                if j < width-1 and not board[i][j+1]:
                    blank += 1

                if blank >= 2:
                    board[i][j] = 2
                    aigo += 1

        if aigo:
            for i in range(height):
                for j in range(width):
                    if board[i][j] == 2:
                        board[i][j] = 0
            aigo = 0

        else:
            break

        turn += 1

    print(turn)


if __name__ == '__main__':
    from io import StringIO
    input = StringIO('''8 9
0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0
0 1 1 0 0 0 1 1 0
0 1 0 1 1 1 0 1 0
0 1 0 0 1 0 0 1 0
0 1 0 1 1 1 0 1 0
0 1 1 0 0 0 1 1 0
0 0 0 0 0 0 0 0 0''').readline

    main()
