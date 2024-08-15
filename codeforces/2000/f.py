from math import inf

testcase_count = int(input())

for _ in range(testcase_count):
    rectangle_count, required_point = map(int, input().split())
    rectangles = [list(map(int, input().split())) for _ in range(rectangle_count)]

    total_gettable = sum(map(sum, rectangles))
    if total_gettable < required_point:
        print("-1")
        continue

    points = 0
    colors = 0
    while points < required_point:
        min_length = inf
        min_rectangle = 0
        min_side = 0
        for i, rectangle in enumerate(rectangles):
            if rectangle[0] != 0 and rectangle[0] < min_length:
                min_rectangle, min_side = i, 0
                min_length = rectangle[0]
            if rectangle[0] != 0 and rectangle[1] < min_length:
                min_rectangle, min_side = i, 1
                min_length = rectangle[1]

        if rectangles[min_rectangle][1 - min_side] == 1:
            points += 1
            rectangles[min_rectangle][min_side] = 0
        rectangles[min_rectangle][1 - min_side] -= 1
        colors += min_length
        points += 1
        print(colors, points, rectangles)

    print(colors)
