height, width = None, None
board = None


def get_deepest(x: int, y: int, been: tuple, deep: int = 1) -> int:
    been += (board[y][x],)

    possibles = list()

    if x > 0 and board[y][x-1] not in been:
        possibles.append((x-1, y))
    if x < width-1 and board[y][x+1] not in been:
        possibles.append((x+1, y))
    if y > 0 and board[y-1][x] not in been:
        possibles.append((x, y-1))
    if y < height-1 and board[y+1][x] not in been:
        possibles.append((x, y+1))

    max_deep = deep
    for possible in possibles:
        x1, y1 = possible
        max_deep = max(max_deep, get_deepest(x1, y1, been, deep + 1))

    return max_deep


def main():
    global height, width, board

    height, width = map(int, input().split())
    board = tuple(input() for _ in range(height))

    print(get_deepest(0, 0, tuple()))


if __name__ == '__main__':
    main()
