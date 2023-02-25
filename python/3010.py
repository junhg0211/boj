def main():
    board = [input() for _ in range(7)]

    result = 0
    for i in range(7):
        for j in range(7):
            if board[i][j] != 'o':
                continue

            if i >= 2 and board[i-1][j] == 'o' and board[i-2][j] == '.':
                result += 1
            if j >= 2 and board[i][j-1] == 'o' and board[i][j-2] == '.':
                result += 1
            if i < 7-2 and board[i+1][j] == 'o' and board[i+2][j] == '.':
                result += 1
            if j < 7-2 and board[i][j+1] == 'o' and board[i][j+2] == '.':
                result += 1

    print(result)



if __name__ == '__main__':
    main()
