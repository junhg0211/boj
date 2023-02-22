from sys import stdin

input = stdin.readline


def main():
    input()
    index = dict()

    for i, number in enumerate(map(int, input().split())):
        if number in index:
            index[number][0] -= 1
        else:
            index[number] = [-1, i, number]

    for frequency, index, number in sorted(index.values()):
        print(f'{number} ' * (-frequency), end='')
    print()


if __name__ == '__main__':
    main()
