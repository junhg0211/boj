def main():
    count = int(input())
    snow = sorted(map(int, input().split()))

    time = 0

    if count > 1:
        while sum(snow) and snow[-2] != 0:
            snow[-1] -= snow[-2]
            time += snow[-2]
            snow[-2] = 0
            snow.sort()
            # print(f'{snow=}, {time=}')

    # print(f'{snow=}, {time=}')
    time += snow[-1]

    if time > 1440:
        print(-1)
    else:
        print(time)


if __name__ == '__main__':
    main()
