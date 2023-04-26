from sys import stdin

input = stdin.readline


def main():
    height, width = map(int, input().split())

    board = [list(map(int, input().rstrip())) for _ in range(height)]
    target = [list(map(int, input().rstrip())) for _ in range(height)]

    count = 0
    for i in range(height - 2):
        for j in range(width - 2):
            if board[i][j] != target[i][j]:
                for k in range(3):
                    for l in range(3):
                        board[i+k][j+l] ^= 1
                count += 1

    for i in range(min(2, width, height)):
        for j in range(width):
            if board[-i-1][j] != target[-i-1][j]:
                count = -1
                break
        if count == -1:
            break

        for j in range(height):
            if board[j][-i-1] != target[j][-i-1]:
                count = -1
                break
        if count == -1:
            break

    print(count)


if __name__ == '__main__':
    main()
