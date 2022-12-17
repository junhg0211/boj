def main():
    _, _, side, star_count = map(int, input().split())
    xs, ys = list(), list()

    for _ in range(star_count):
        x, y = map(int, input().split())
        xs.append(x)
        ys.append(y)

    max_block = 0
    for x in xs:
        for y in ys:
            blocks = [0, 0, 0, 0]
            for i in range(star_count):
                conditions = (
                    x <= xs[i] <= x+side,
                    y <= ys[i] <= y+side,
                    x-side <= xs[i] <= x,
                    y-side <= ys[i] <= y
                )
                if conditions[0] and conditions[1]:
                    blocks[0] += 1
                if conditions[0] and conditions[3]:
                    blocks[1] += 1
                if conditions[2] and conditions[1]:
                    blocks[2] += 1
                if conditions[2] and conditions[3]:
                    blocks[3] += 1
            max_block = max(max(blocks), max_block)

    print(star_count - max_block)

if __name__ == '__main__':
    main()

