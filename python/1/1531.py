def main():
    n, m = map(int, input().split())
    board = [[0 for _ in range(100)] for _ in range(100)]

    for _ in range(n):
        x1, y1, x2, y2 = map(int, input().split())

        for i in range(y1-1, y2):
            for j in range(x1-1, x2):
                board[i][j] += 1

    count = 0
    for i in range(100):
        for j in range(100):
            if board[i][j] > m:
                count += 1

    print(count)


if __name__ == '__main__':
    main()
