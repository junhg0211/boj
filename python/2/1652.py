def transpose(size: int, board: list) -> list:
    result = list()
    for i in range(size):
        line = ''.join(board[j][i] for j in range(size))
        result.append(line)
    return result


def get_places(board: list):
    result = 0
    for line in board:
        result += len(tuple(filter(lambda x: len(x) >= 2, line.split('X'))))
    return result


def main():
    size = int(input())

    board = [input() for _ in range(size)]
    print(get_places(board), end=' ')

    board = transpose(size, board)
    print(get_places(board))


if __name__ == '__main__':
    main()
