from collections import deque
from copy import copy
from math import hypot, sqrt


def get_triangle_area(point1, point2, point3) -> float:
    a = hypot(point1[0] - point2[0], point1[1] - point2[1])
    b = hypot(point2[0] - point3[0], point2[1] - point3[1])
    c = hypot(point3[0] - point1[0], point3[1] - point1[1])
    s = (a + b + c) / 2
    return sqrt(s * (s-a) * (s-b) * (s-c))


def get_area(indeces: set, positions: list) -> float:
    area = 0.0
    indeces = list(indeces)

    point1 = positions[indeces[0]]
    for i in range(1, len(indeces)-1):
        point2 = positions[indeces[i]]
        point3 = positions[indeces[i+1]]

        area += get_triangle_area(point1, point2, point3)

    return area


def main():
    count = int(input())
    positions = [tuple(map(int, input().split())) for _ in range(count)]

    result = 0

    queue = deque([set(range(count))])
    maxes = list()
    while queue:
        indeces = queue.popleft()

        if len(indeces) == 3:
            result = max(result, get_area(indeces, positions))
            continue

        max_area = 0
        maxes.clear()
        for index in indeces:
            new_indeces = copy(indeces)
            new_indeces.remove(index)

            area = get_area(new_indeces, positions)
            if area > max_area:
                maxes.clear()
                maxes.append(new_indeces)
                max_area = area
            elif area == max_area:
                maxes.append(new_indeces)

        queue.extend(maxes)

    print(format(result, '.9f'))


if __name__ == '__main__':
    main()
