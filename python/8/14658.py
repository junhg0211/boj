def main():
    _, _, side, star_count = map(int, input().split())
    stars = [tuple(map(int, input().split())) for _ in range(star_count)]

    max_block = 0
    for star in stars:
        blocks = [0, 0, 0, 0]
        for star2 in stars:
            conditions = (
                0 <= star2[0] - star[0] <= side,
                0 <= star2[1] - star[1] <= side,
                0 <= star[0] - star2[0] <= side,
                0 <= star[1] - star2[1] <= side
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
