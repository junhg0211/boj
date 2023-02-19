from collections import deque
from sys import stdin

input = stdin.readline


def main():
    width, depth, height = map(int, input().split())
    box = list()

    process = deque()

    for i in range(height):
        plate = list()
        for j in range(depth):
            row = list(map(int, input().split()))
            plate.append(row)
            for k, state in enumerate(row):
                if state == 1:
                    process.append((i, j, k))
        box.append(plate)

    if not process:
        print('-1')
        return

    days = 0
    next_process = deque()
    while process or next_process:
        if not process:
            process, next_process = next_process, deque()
            days += 1
        if not process:
            days -= 1
            break

        x, y, z = process.popleft()

        if x > 0 and box[x-1][y][z] == 0:
            box[x-1][y][z] = 1
            next_process.append((x-1, y, z))
        if x < height-1 and box[x+1][y][z] == 0:
            box[x+1][y][z] = 1
            next_process.append((x+1, y, z))
        if y > 0 and box[x][y-1][z] == 0:
            box[x][y-1][z] = 1
            next_process.append((x, y-1, z))
        if y < depth-1 and box[x][y+1][z] == 0:
            box[x][y+1][z] = 1
            next_process.append((x, y+1, z))
        if z > 0 and box[x][y][z-1] == 0:
            box[x][y][z-1] = 1
            next_process.append((x, y, z-1))
        if z < width-1 and box[x][y][z+1] == 0:
            box[x][y][z+1] = 1
            next_process.append((x, y, z+1))

    not_rippen = False
    for plate in box:
        for row in plate:
            if 0 in row:
                not_rippen = True
                break
        if not_rippen:
            break

    if not_rippen:
        print('-1')
    else:
        print(days)


if __name__ == '__main__':
    main()
