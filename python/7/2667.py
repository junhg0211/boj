def pop_neighbor(board: list, x: int, y: int, size: int):
    count = 1
    board[y][x] = 0

    if x > 0 and board[y][x-1] == 1:
        count += pop_neighbor(board, x-1, y, size)
    if x < size-1 and board[y][x+1] == 1:
        count += pop_neighbor(board, x+1, y, size)
    if y > 0 and board[y-1][x] == 1:
        count += pop_neighbor(board, x, y-1, size)
    if y < size-1 and board[y+1][x] == 1:
        count += pop_neighbor(board, x, y+1, size)

    return count


def main():
    size = int(input())
    board = [list(map(int, input())) for _ in range(size)]

    neighbors = list()

    for y in range(size):
        for x in range(size):
            if board[y][x] == 1:
                neighbors.append(pop_neighbor(board, x, y, size))

    print(len(neighbors))
    print(*sorted(neighbors), sep='\n')


if __name__== '__main__':
    main()
