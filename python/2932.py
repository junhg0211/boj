def main():
    size, count = map(int, input().split())

    positions = dict()
    moves = list()

    for _ in range(count):
        move = list(map(lambda x: int(x) - 1, input().split()))
        moves.append(move)

        x, r, c = move
        positions[x] = list(divmod(x, size))

    print(positions)

    for x, r, c in moves:
        ty, tx = positions[x]
        xr = (c - tx) % size
        yr = (r - ty) % size

        print(xr, yr, ty, tx)
        # print(xr + yr)

        for key, value in positions.items():
            if value[0] == ty:
                value[1] += xr
                value[1] %= size
                tx += xr
                tx %= size
            if value[1] == tx:
                value[0] += yr
                value[0] %= size

        print(positions)


if __name__ == '__main__':
    main()
