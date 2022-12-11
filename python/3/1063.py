def position_by_coordinate(coordinate: list[int, int]) -> str:
    x, y = coordinate
    x = 'ABCDEFGH'[x]
    y = str(8-y)
    return x + y


def coordinate_by_position(position: str) -> list[int, int]:
    x, y = position
    x = 'ABCDEFGH'.index(x)
    y = 8-int(y)
    return [x, y]


def main():
    king, stone, count = input().split()
    king = coordinate_by_position(king)
    stone = coordinate_by_position(stone)

    for _ in range(int(count)):
        # print(king, stone)
        move = input()

        dx = 0
        dy = 0
        if move[0] == 'L':
            dx = -1
        if move[0] == 'R':
            dx = 1
        if move == 'T' or len(move) > 1 and move[1] == 'T':
            dy = -1
        if move == 'B' or len(move) > 1 and move[1] == 'B':
            dy = 1

        kx = king[0]
        ky = king[1]
        sx = stone[0]
        sy = stone[1]
        if dx == -1 and kx == 0 \
                or dx == 1 and kx == 7 \
                or dy == -1 and ky == 0 \
                or dy == 1 and ky == 7:
            # print(1)
            continue

        kx += dx
        ky += dy

        if kx != sx or ky != sy:
            king[0] = kx
            king[1] = ky
            # print(2)
            continue

        if dx == -1 and sx == 0 \
                or dx == 1 and sx == 7 \
                or dy == -1 and sy == 0 \
                or dy == 1 and sy == 7:
            # print(3)
            continue

        stone[0] += dx
        stone[1] += dy
        king[0] = kx
        king[1] = ky

    # print(king, stone)
    print(position_by_coordinate(king))
    print(position_by_coordinate(stone))



if __name__ == '__main__':
    main()
