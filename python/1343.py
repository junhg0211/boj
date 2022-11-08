def main():
    board = input()

    result = list()
    while board:
        if board.startswith('XXXX'):
            result.append('AAAA')
            board = board[4:]
        elif board.startswith('XX'):
            result.append('BB')
            board = board[2:]
        elif board.startswith('.'):
            result.append('.')
            board = board[1:]
        else:
            print(-1)
            return

    print(''.join(result))

if __name__ == '__main__':
    main()
