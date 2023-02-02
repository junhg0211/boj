from sys import stdin

input = stdin.readline


def main():
    width, height = map(int, input().split())
    unsafe = set()

    queens_raw = map(lambda x: int(x) - 1, input().split())
    queen_count = next(queens_raw)+1
    queens = list()
    for _ in range(queen_count):
        position = (next(queens_raw), next(queens_raw))
        queens.append(position)
        unsafe.add(position)

    knights_raw = map(lambda x: int(x) - 1, input().split())
    knight_count = next(knights_raw) + 1
    knights = list()
    for _ in range(knight_count):
        position = (next(knights_raw), next(knights_raw))
        knights.append(position)
        unsafe.add(position)

    pawns_raw = map(lambda x: int(x) - 1, input().split())
    pawn_count = next(pawns_raw)+1
    pawns = list()
    for _ in range(pawn_count):
        position = (next(pawns_raw), next(pawns_raw))
        pawns.append(position)
        unsafe.add(position)

    for x, y in queens:
        for i in range(x, -1, -1):
            position = (i, y)
            if position in pawns or position in knights:
                break
            unsafe.add(position)
        for i in range(x, width):
            position = (i, y)
            if position in pawns or position in knights:
                break
            unsafe.add(position)
        for i in range(y, -1, -1):
            position = (x, i)
            if position in pawns or position in knights:
                break
            unsafe.add(position)
        for i in range(y, height):
            position = (x, i)
            if position in pawns or position in knights:
                break
            unsafe.add(position)

        for i in range(min(x+1, y+1)):
            position = (x-i, y-i)
            if position in pawns or position in knights:
                break
            unsafe.add(position)
        for i in range(min(width-x, height-y)):
            position = (x+i, y+i)
            if position in pawns or position in knights:
                break
            unsafe.add(position)
        for i in range(min(x+1, height-y)):
            position = (x-i, y+i)
            if position in pawns or position in knights:
                break
            unsafe.add(position)
        for i in range(min(width-x, y+1)):
            position = (x+i, y-i)
            if position in pawns or position in knights:
                break
            unsafe.add(position)

    for x, y in knights:
        candidates = [
            (x-2, y-1), (x-1, y-2),
            (x-2, y+1), (x-1, y+2),
            (x+2, y-1), (x+1, y-2),
            (x+2, y+1), (x+1, y+2)
        ]
        for candidate in candidates:
            xc, yc = candidate
            if 0 <= xc <= width-1 and 0 <= yc <= height-1:
                unsafe.add(candidate)

    print(width * height - len(unsafe))


if __name__ == '__main__':
    main()
