def main():
    count = int(input())

    councils = list()
    for _ in range(count):
        councils.append(tuple(map(int, input().split())))
    councils.sort(key=lambda x: (x[1], x[0]))

    end_time = 0
    count = 0
    for council in councils:
        if council[0] >= end_time:
            count += 1
            end_time = council[1]

    print(count)


if __name__ == '__main__':
    main()
