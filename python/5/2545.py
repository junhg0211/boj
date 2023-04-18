from sys import stdin

input = stdin.readline


def tick():
    input()
    a, b, c, count = map(int, input().split())
    ns = sorted((a, b, c))

    if not count:
        return ns

    if ns[-1] != ns[-2]:
        diff = ns[-1] - ns[-2]

        if diff > count:
            ns[-1] -= count
            return ns

        ns[-1] -= diff
        count -= diff

        if not count:
            return ns

    if ns[-2] != ns[-3]:
        diff = ns[-2] - ns[-3]

        if diff * 2 > count:
            if count % 2 == 0:
                ns[-1] -= count//2
                ns[-2] -= count//2
                return ns

            ns[-1] -= count//2
            ns[-2] -= count//2 + 1

            return ns

        ns[-2] = ns[-3]
        ns[-1] = ns[-3]
        count -= diff * 2

        if not count:
            return ns

    ns[-1] -= count//3
    ns[-2] -= count//3
    ns[-3] -= count//3
    count %= 3

    i = 0
    while count:
        ns[i] -= 1
        count -= 1
        i += 1

    return ns


def main():
    for _ in range(int(input())):
        a, b, c = tick()
        print(a * b * c)


if __name__ == '__main__':
    main()
