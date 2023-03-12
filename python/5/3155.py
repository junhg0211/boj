from math import inf
from sys import stdin

input = stdin.readline

width = None
ups = None
downs = None


def can_next(x, y):
    if y >= ups[x+1]:
        return False
    if y <= downs[x+1]:
        return False
    return True


def generator():
    yield 0
    i = 1
    while True:
        yield -i
        yield i
        i += 1


def tick(y):
    length = 0
    x = 0

    route = [y]

    while x < width-1:
        for dy in generator():
            if can_next(x, y + dy):
                x += 1
                length += 1 + dy
                y += dy
                route.append(y)
                break

    return length, ' '.join(map(str, route))


def main():
    global width, ups, downs
    width = int(input())
    ups = list(map(int, input().split()))
    downs = list(map(int, input().split()))

    route_length = inf
    route = None
    for i in range(downs[0] + 1, ups[0]):
        length, a_route = tick(i)
        if length < route_length:
            route_length = length
            route = a_route

    print(route)


if __name__ == '__main__':
    main()
