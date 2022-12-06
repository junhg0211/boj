from sys import stdin

input = stdin.readline


def possibilities(count: int, nos: dict):
    result = list()
    for i in range(1, 2**count):
        yes = True
        j = count-1

        while i:
            if i & 1:
                if j in nos:
                    for no in nos[j]:
                        if no in result:
                            yes = False
                            break
                result.append(j)
            i >>= 1
            j -= 1

        if yes:
            yield result
        result.clear()


def overlap(c1, c2):
    return c1[0] < c2[0] < c1[1] or c1[0] < c2[1] < c1[1] \
            or c2[0] < c1[0] < c2[1] or c2[0] < c1[1] < c2[1]


def main():
    count = int(input())
    councils = list()
    for _ in range(count):
        councils.append(tuple(map(int, input().split())))

    nos = dict()
    for i in range(count):
        for j in range(i+1, count):
            if not overlap(councils[i], councils[j]):
                continue

            if i in nos:
                nos[i].append(j)
            else:
                nos[i] = [j]

    max_councils = 0
    for possibility in possibilities(count, nos):
        max_councils = max(max_councils, len(possibility))

    print(max_councils)


if __name__ == '__main__':
    main()
