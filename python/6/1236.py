def main():
    height, width = map(int, input().split())
    board = [list(map(lambda x: x == 'X', input())) for _ in range(height)]
    rows = sum(int(not any(board[i])) for i in range(height))
    columns = sum(int(not any(map(lambda j: board[j][i], range(height)))) for i in range(width))

    print(max(rows, columns))


if __name__ == '__main__':
    main()
