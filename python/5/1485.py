from math import hypot


def distance(points, index1, index2):
    point1 = points[index1]
    point2 = points[index2]

    dx = point1[0] - point2[0]
    dy = point1[1] - point2[1]

    return hypot(dx, dy)


def tick():
    points = [tuple(map(int, input().split())) for _ in range(4)]

    distances = sorted((
        distance(points, 0, 1),
        distance(points, 0, 2),
        distance(points, 0, 3)
    ))
    a, b, c = distances

    if not (a == b < c):
        return 2

    x = a
    y = c

    distances = sorted((
        distance(points, 1, 0),
        distance(points, 1, 2),
        distance(points, 1, 3)
    ))
    a, b, c = distances

    if not (a == b < c):
        return 3
    if not (x == a and y == c):
        return 4

    distances = sorted((
        distance(points, 2, 0),
        distance(points, 2, 1),
        distance(points, 2, 3)
    ))
    a, b, c = distances

    if not (a == b < c):
        return 5
    if not (x == a and y == c):
        return 6

    distances = sorted((
        distance(points, 3, 0),
        distance(points, 3, 1),
        distance(points, 3, 2)
    ))
    a, b, c = distances

    if not (a == b < c):
        return 7
    if not (x == a and y == c):
        return 8

    return 1


def main():
    for _ in range(int(input())):
        value = tick()
        if value != 1:
            print(0)
        else:
            print(1)


if __name__ == '__main__':
    main()
