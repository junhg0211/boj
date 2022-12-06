from sys import stdin

input = stdin.readline


def possibilities(count: int):
    result = set()
    for i in range(1, 2**count):
        j = 0
        while i:
            if i & 1:
                result.add(j)
            i >>= 1
            j += 1
        yield result
        result.clear()


def overlap(c1, c2):
    return c1[0] < c2[0] < c1[1] or c1[0] < c2[1] < c1[1] \
            or c2[0] < c1[0] < c2[1] or c2[0] < c1[1] < c2[1]


def is_valid(councils: list, selections: list):
    selecteds = tuple(map(lambda x: councils[x], selections))

    for i in range(len(selecteds)):
        for j in range(i+1, len(selecteds)):
            if overlap(selecteds[i], selecteds[j]):
                return False

    return True


def main():
    count = int(input())

    councils = list()
    for _ in range(count):
        councils.append(tuple(map(int, input().split())))

    max_councils = 0
    for possibility in possibilities(count):
        if is_valid(councils, possibility):
            if (candidate := len(possibility)) > max_councils:
                max_councils = candidate

    print(max_councils)


if __name__ == '__main__':
    main()
