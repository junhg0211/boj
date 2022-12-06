from copy import copy
from math import hypot, inf


def permutation(numbers):
    if len(numbers) == 1:
        return [numbers]

    result = list()

    for i in range(len(numbers)):
        clone = numbers.copy()

        value = clone.pop(i)
        for p in permutation(clone):
            result.append([value] + p)

    return result


def main():
    coords = list(map(int, input().split()))

    slope1 = (coords[1] - coords[3]) / (coords[0] - coords[2]) if coords[0] != coords[2] else inf
    slope2 = (coords[1] - coords[5]) / (coords[0] - coords[4]) if coords[0] != coords[4] else inf
    slope3 = (coords[3] - coords[5]) / (coords[2] - coords[4]) if coords[2] != coords[4] else inf

    if slope1 == slope2 == slope3:
        print(-1)
        return

    min_distance = inf
    max_distance = 0
    for p in permutation([0, 2, 4]):
        distance = hypot(coords[p[0]]-coords[p[1]], coords[p[0]+1]-coords[p[1]+1])
        distance += hypot(coords[p[1]]-coords[p[2]], coords[p[1]+1]-coords[p[2]+1])
        if distance > max_distance:
            max_distance = distance
        if distance < min_distance:
            min_distance = distance

    print(2 * (max_distance - min_distance))


if __name__ == '__main__':
    main()
