from sys import stdin

input = stdin.readline


def main():
    size, sum_count = map(int, input().split())
    numbers = list()

    for i in range(size):
        row = list()
        for j, number in enumerate(map(int, input().split())):
            delta = numbers[i-1][j] if i > 0 else 0
            mae = row[j-1] if j > 0 else 0
            diagonal = numbers[i-1][j-1] if i > 0 and j > 0 else 0

            row.append(delta + mae + number - diagonal)
        numbers.append(row)

    for _ in range(sum_count):
        y1, x1, y2, x2 = map(lambda x: int(x) - 1, input().split())

        result = numbers[y2][x2]

        if x1 > 0:
            result -= numbers[y2][x1 - 1]
        if y1 > 0:
            result -= numbers[y1 - 1][x2]
        if x1 > 0 and y1 > 0:
            result += numbers[y1-1][x1-1]

        print(result)


if __name__ == '__main__':
    main()
