def main():
    height, width = map(int, input().split())

    board = list()
    for _ in range(height):
        board.append(input())
    input()

    error = 0
    for i in range(height):
        line = input()

        for j, letter in enumerate(line):
            if board[i][j] == letter:
                error += 1

    print(error)


if __name__ == "__main__":
    main()
