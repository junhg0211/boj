from math import inf


def tick():
    width, height = map(int, input().split())
    board = list()
    for _ in range(height):
        board.append(list(map(int, input().split())))

    max_column = 0
    max_value = -inf
    for i in range(width):
        value = 1
        for j in range(height):
            value *= board[j][i]

        if value >= max_value:
            max_column = i
            max_value = value

    print(max_column + 1)


def main():
    testcase_count = int(input())

    for _ in range(testcase_count):
        tick()


if __name__ == '__main__':
    main()
