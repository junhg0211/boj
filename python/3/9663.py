def get_possibles(positions: tuple, size: int):
    i = len(positions)

    for j in range(size):
        valid = True
        for x, y in positions:
            if x == i or y == j or i-j == x-y or size-i-1 - j == size-x-1 - y:
                valid = False
                break
        if valid:
            yield (i, j)


def get_count(positions: tuple, left: int, size: int, sy: int = 0):
    if left == 0:
        print(positions)
        return 1

    result = 0
    for possible in get_possibles(positions, size):
        result += get_count(positions + (possible,), left-1, size, possible[0])

    return result


def main(size: int):
    return get_count(tuple(), size, size)


if __name__ == '__main__':
    with open('9663_1.py', 'w') as file:
        for i in range(1, 15):
            file.write(f'{main(i)}\n')
