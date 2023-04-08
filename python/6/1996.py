from sys import stdin

input = stdin.readline


def main():
    size = int(input())

    board = tuple(tuple(map(int, input().rstrip().replace('.', '0'))) for _ in range(size))

    for i in range(size):
        for j in range(size):
            if board[i][j] != 0:
                print('*', end='')
                continue

            sum_ = 0
            if i > 0:
                if j > 0:
                    sum_ += board[i-1][j-1]
                if j < size-1:
                    sum_ += board[i-1][j+1]
                sum_ += board[i-1][j]
            if i < size-1:
                if j > 0:
                    sum_ += board[i+1][j-1]
                if j < size-1:
                    sum_ += board[i+1][j+1]
                sum_ += board[i+1][j]
            if j > 0:
                sum_ += board[i][j-1]
            if j < size-1:
                sum_ += board[i][j+1]

            print('M' if sum_ >= 10 else sum_, end='')

        print()


if __name__ == '__main__':
    main()
