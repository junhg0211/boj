from sys import stdin

input = stdin.readline


def main():
    count = int(input())

    councils = list()
    for _ in range(count):
        councils.append([tuple(map(int, input().split())), 0])
    councils.sort(key=lambda x: x[0])

    total_max = 0
    for i in range(count-1, -1, -1):
        council = councils[i][0]
        possible_max = 1
        for j in range(i+1, count):
            other, possible = councils[j]
            if council[1] <= other[0]:
                possible_max = max(possible_max, 1 + possible)
        councils[i][1] = possible_max
        total_max = max(total_max, possible_max)

    print(total_max)


if __name__ == '__main__':
    main()
