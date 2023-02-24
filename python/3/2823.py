def main():
    height, width = map(int, input().split())
    board = [input() for _ in range(height)]

    end = False
    possible = True
    for i in range(height):
        for j in range(width):
            if board[i][j] == 'X':
                continue

            possibles = 4
            if i < 1 or board[i-1][j] == 'X':
                possibles -= 1
            if j < 1 or board[i][j-1] == 'X':
                possibles -= 1
            if i >= height - 1 or board[i+1][j] == 'X':
                possibles -= 1
            if j >= width - 1 or board[i][j+1] == 'X':
                possibles -= 1

            if possibles <= 1:
                possible = False
                end = True

        if end:
            break

    if possible:
        print('0')
    else:
        print('1')


if __name__ == '__main__':
    main()
