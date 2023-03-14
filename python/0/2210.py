board = None
cache = dict()


def get(x, y, length):
    if length == 1:
        return (board[y][x],)

    seed = (x, y, length)
    if seed in cache:
        return cache[seed]

    result = list()
    if x > 0:
        for string in get(x-1, y, length-1):
            result.append(string + board[y][x])
    if x < 4:
        for string in get(x+1, y, length-1):
            result.append(string + board[y][x])
    if y > 0:
        for string in get(x, y-1, length-1):
            result.append(string + board[y][x])
    if y < 4:
        for string in get(x, y+1, length-1):
            result.append(string + board[y][x])

    cache[seed] = result
    return result


def main():
    global board

    board = [list(input().split()) for _ in range(5)]

    been = set()

    for i in range(5):
        for j in range(5):
            for string in get(j, i, 6):
                been.add(string)
                '''
                if string not in been:
                    been.add(string)
                    print(string)
                    '''

    print(len(been))


if __name__ == '__main__':
    main()
