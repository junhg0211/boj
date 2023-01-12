def main():
    _ = (input())
    buildings = list()

    for i, height in enumerate(map(int, input().split())):
        while buildings and buildings[-1][0] < height:
            buildings.pop()

        if not buildings:
            print('0', end=' ')
        else:
            print(buildings[-1][1], end=' ')

        buildings.append((height, i+1))


if __name__ == '__main__':
    main()
