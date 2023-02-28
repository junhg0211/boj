def print_sub(x, y, index, board):
    print(f'#{index}. {board[y+1][x+1]}')
    goals = [
        board[y][x],
        board[y][x+1],
        board[y][x+2],
        board[y+1][x+2],
        board[y+2][x+2],
        board[y+2][x+1],
        board[y+2][x],
        board[y+1][x]
    ]
    goals.sort()

    for i, goal in enumerate(goals):
        print(f'#{index}-{i+1}. {goal}')


def main():
    board = [input().split() for _ in range(9)]

    goals = [
        (board[1][1], 0, 0),
        (board[1][4], 3, 0),
        (board[1][7], 6, 0),
        (board[4][7], 6, 3),
        (board[7][7], 6, 6),
        (board[7][4], 3, 6),
        (board[7][1], 0, 6),
        (board[4][1], 0, 3),
    ]
    goals.sort()

    for i, goal in enumerate(goals):
        print_sub(goal[1], goal[2], i+1, board)


if __name__ == '__main__':
    main()
