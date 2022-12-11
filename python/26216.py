def get_address(number, blues, depth):
    blue = blues + 1

    result = list()
    for i in range(depth, -1, -1):
        result.append((number) // blue**i * blue**i)

    while result[-2] == result[-3] and len(result) > 3:
        result.pop(-2)

    return result


def reversed_index(values, value):
    return len(values) - next(i for i, val in enumerate(reversed(values)) if val == value) - 1


def get_distance(blues, depth, oa, ob):
    b = get_address(ob, blues, depth)
    a = get_address(oa, blues, depth)

    for i, position in enumerate(reversed(b)):
        if position in a:
            print(a, b, position, i)
            return i, len(a) - (len(b)-i)

def main():
    blues, depth, queries = map(int, input().split())

    for _ in range(queries):
        a, b = map(int, input().split())
        print(get_distance(blues, depth, a, b))


if __name__ == '__main__':
    main()
    '''
    for i in range(1, 3**3):
        print(i, get_address(i, 2, 3))
        '''
