from pprint import pprint

farm = list()


def recursive_search(x_, y_, direction=0):
    if farm[y_][x_]:
        farm[y_][x_] = False
    else:
        return

    up_available = y_ > 0
    down_available = y_ < height - 1
    left_available = x_ > 0
    right_available = x_ < width - 1

    if direction == 0:  # UP
        straight = x_, y_ - 1 if up_available else None
        right = x_ + 1, y_ if right_available else None
        left = x_ - 1, y_ if left_available else None
    elif direction == 1:  # RIGHT
        straight = x_ + 1, y_ if right_available else None
        right = x_, y_ + 1 if down_available else None
        left = x_, y_ - 1 if up_available else None
    elif direction == 2:  # DOWN
        straight = x_, y_ + 1 if down_available else None
        right = x_ - 1, y_ if left_available else None
        left = x_ + 1, y_ if right_available else None
    else:  # LEFT
        straight = x_ - 1, y_ if left_available else None
        right = x_, y_ - 1 if up_available else None
        left = x_, y_ + 1 if down_available else None

    if None not in straight:
        recursive_search(*straight, direction)
    if None not in right:
        recursive_search(*right, (direction + 1) % 4)
    if None not in left:
        recursive_search(*left, (direction - 1) % 4)


for _ in range(int(input())):
    width, height, count = map(int, input().split())
    for _ in range(height):
        line = list()
        for _ in range(width):
            line.append(False)
        farm.append(line)
    for _ in range(count):
        x, y = map(int, input().split())
        farm[y][x] = True

    loafs = 0
    for y in range(height):
        for x in range(width):
            if not farm[y][x]:
                continue

            loafs += 1
            recursive_search(x, y)
            farm[y][x] = True
            recursive_search(x, y, 1)
            farm[y][x] = True
            recursive_search(x, y, 2)
            farm[y][x] = True
            recursive_search(x, y, 3)

    print(loafs)
