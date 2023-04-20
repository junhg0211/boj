from sys import stdin

input = stdin.readline


def main():
    width, height = map(int, input().split())

    cuts = [[0, height], [0, width]]

    for _ in range(int(input())):
        i, place = map(int, input().split())
        cuts[i].append(place)

    cuts[0].sort()
    cuts[1].sort()

    horizontal = 0
    for i in range(len(cuts[0]) - 1):
        horizontal = max(cuts[0][i+1] - cuts[0][i], horizontal)

    vertical = 0
    for i in range(len(cuts[1]) - 1):
        vertical = max(cuts[1][i+1] - cuts[1][i], vertical)

    print(horizontal * vertical)


if __name__ == '__main__':
    main()
