from sys import stdin

input = stdin.readline


def main():
    height, width = map(int, input().split())
    board = list()

    for i in range(height):
        line = list()
        for j, number in enumerate(map(int, input().split())):
            if i == j == 0:
                line.append(number)
                continue

            if i == 0:
                line.append(number + line[j-1])
                continue

            if j == 0:
                line.append(number + board[i-1][j])
                continue

            line.append(number + line[j-1] + board[i-1][j] - board[i-1][j-1])

        board.append(line)

    for _ in range(int(input())):
        y1, x1, y2, x2 = map(lambda x: int(x) - 1, input().split())

        result = board[y2][x2]

        if x1 > 0:
            result -= board[y2][x1-1]

        if y1 > 0:
            result -= board[y1-1][x2]

            if x1 > 0:
                result += board[y1-1][x1-1]

        print(result)

if __name__ == '__main__':
    main()
