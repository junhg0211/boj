from sys import stdin

input = stdin.readline


def main():
    # -- get data
    row_count, _ = map(int, input().split())

    rows = list()
    for _ in range(row_count):
        rows.append(input().strip())

    change_count = int(input())

    # -- get row count
    row_same_count = dict()
    for row in rows:
        zero_count = row.count('0')
        if change_count >= zero_count and change_count % 2 == zero_count % 2:
            row_same_count[row] = row_same_count.get(row, 0) + 1

    if row_same_count:
        print(max(row_same_count.values()))
    else:
        print('0')


if __name__ == '__main__':
    main()
