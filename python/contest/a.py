def main():
    board = list()
    for _ in range(10):
        board.append(input().split())

    for i in range(10):
        bad = False
        for j in range(1, 10):
            if board[i][0] != board[i][j]:
                bad = True
                break

        if not bad:
            print("1")
            return

    for i in range(10):
        bad = False
        for j in range(1, 10):
            if board[0][i] != board[j][i]:
                bad = True
                break

        if not bad:
            print("1")
            return

    print("0")


if __name__ == "__main__":
    main()
