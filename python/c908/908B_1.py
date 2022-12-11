cache = dict()


def get_address(number, blues, depth):
    if number in cache:
        return cache[number]

    result = list()
    blues += 1
    for i in range(depth, -1, -1):
        bluesi = blues**i
        if number % bluesi == 0:
            result.append(number // bluesi * 2 + 1)
            return result
        else:
            result.append(number // bluesi)

    cache[number] = result
    return result


def main():
    blues, depth, queries = map(int, input().split())

    max_value = (blues+1)**depth

    for _ in range(queries):
        a, b = map(int, input().split())

        if b+1 > max_value:
            print('-1')
            continue

        if a == b:
            print('0')
            continue

        a = get_address(a, blues, depth)
        b = get_address(b, blues, depth)

        for i, position in enumerate(b):
            if position != a[i]:
                break

        print((len(a)-i) + (len(b)-i))


if __name__ == '__main__':
    main()
