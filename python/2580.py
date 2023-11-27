def remove(from_, obj):
    for i, value in enumerate(from_):
        if value == obj:
            return from_.pop(i)


def print_board(board):
    for row in board:
        print(*row)


def main():
    # -- get board
    board = list()
    zeros = list()
    for i in range(9):
        row = list(map(int, input().split()))

        for j, number in enumerate(row):
            if number == 0:
                zeros.append((i, j))

        board.append(row)

    assertions = list()
    nos = set()
    index = 0
    possibles = list()
    while index < len(zeros):
        i, j = zeros[index]

        # -- get possibilities
        # remove possibles and fill with (1..=9)
        possibles.clear()
        for k in range(1, 10):
            possibles.append(k)
        # get possibles
        for k in range(9):
            # remove from same row and column
            remove(possibles, board[i][k])
            remove(possibles, board[k][j])

            # remove from same subsquare
            y = i//3*3 + k//3
            x = j//3*3 + k%3
            remove(possibles, board[y][x])

            # remove from nos
            k += 1
            if (index, k) in nos:
                remove(possibles, k)

        # -- if `possibles` is not empty, select one and push stack
        if possibles:
            assertions.append((index, possibles[0]))
            board[i][j] = possibles[0]

            # process next loop
            index += 1
            continue

        # -- if `possibles` is empty, pop stack and process next scenario
        # remove this nos
        for k in range(1, 10):
            nos -= {(index, k)}

        # pop from stack
        new_index, thing = assertions.pop()
        y, x = zeros[new_index]
        board[y][x] = 0
        nos.add((new_index, thing))
        index = new_index

    print_board(board)


if __name__ == '__main__':
    main()
