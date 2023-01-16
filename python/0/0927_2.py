def main():
    counts = dict()

    for _ in range(int(input())):
        key, value = input().split()
        value = int(value)

        if key in counts:
            counts[key] += value
        else:
            counts[key] = value

    if 5 in counts.values():
        print('YES')
    else:
        print('NO')


if __name__ == '__main__':
    main()
