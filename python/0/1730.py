board = None


def update_cell(x, y, direction):
    if direction in 'UD':
        if board[y][x] == '.':
            board[y][x] = '|'
        elif board[y][x] == '-':
            board[y][x] = '+'
    elif direction in 'LR':
        if board[y][x] == '.':
            board[y][x] = '-'
        elif board[y][x] == '|':
            board[y][x] = '+'


def main():
    global board

    size = int(input())

    board = [['.' for _ in range(size)] for _ in range(size)]

    x, y = 0, 0

    for direction in input():
        if direction == 'U' and y > 0:
            update_cell(x, y, direction)
            y -= 1
            update_cell(x, y, direction)

        if direction == 'L' and x > 0:
            update_cell(x, y, direction)
            x -= 1
            update_cell(x, y, direction)

        if direction == 'D' and y < size-1:
            update_cell(x, y, direction)
            y += 1
            update_cell(x, y, direction)

        if direction == 'R' and x < size-1:
            update_cell(x, y, direction)
            x += 1
            update_cell(x, y, direction)

    for line in board:
        print(''.join(line))


if __name__ == '__main__':
    main()
