from math import hypot, sqrt


def get_triangle_area(point1, point2, point3) -> float:
    a = hypot(point1[0] - point2[0], point1[1] - point2[1])
    b = hypot(point2[0] - point3[0], point2[1] - point3[1])
    c = hypot(point3[0] - point1[0], point3[1] - point1[1])
    s = (a + b + c) / 2
    return sqrt(s * (s-a) * (s-b) * (s-c))


def main():
    count = int(input())
    positions = [tuple(map(int, input().split())) for _ in range(count)]

    result = 0
    for i in range(count):
        for j in range(i+1, count):
            for k in range(j+1, count):
                area = get_triangle_area(positions[i], positions[j], positions[k])
                result = max(result, area)

    print(result)


if __name__ == '__main__':
    main()
