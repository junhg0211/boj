def main():
    board = list()
    for _ in range(4):
        board.append(input())

    distance = 0
    for i in range(4):
        for j in range(4):
            c = board[i][j]

            if c == '.':
                continue

            index = ord(c) - ord('A')
            correct_row = index // 4
            correct_col = index % 4

            distance += abs(i - correct_row) + abs(j - correct_col)

    print(distance)


if __name__ == "__main__":
    main()
