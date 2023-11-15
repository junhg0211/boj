from heapq import heappush, heappop
from math import hypot

parent_map = dict()


def get_ancient(of: int):
    this = of
    while this in parent_map:
        this = parent_map[this]

    if of != this:
        parent_map[of] = this

    return this


def main():
    points = list()
    for _ in range(point_count := int(input())):
        points.append(tuple(map(float, input().split())))

    distances = list()
    for i in range(point_count):
        for j in range(i+1, point_count):
            dx = points[i][0] - points[j][0]
            dy = points[i][1] - points[j][1]
            distance = hypot(dx, dy)

            heappush(distances, (distance, i, j))

    connection_count = 0
    result = 0
    while connection_count < point_count-1:
        data = heappop(distances)
        distance, i, j = data

        ip = get_ancient(i)
        jp = get_ancient(j)
        if ip == jp:
            continue

        result += distance
        parent_map[ip] = jp
        connection_count += 1
        """
        print('connect', i, j)
        print('ancients', ip, jp)
        print('parent map', parent_map)
        """

    print(f'{result:.2}')
    # print(result)


if __name__ == '__main__':
    main()
