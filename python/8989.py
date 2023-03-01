def get_angle(time):
    hour, minute = time
    hour %= 12

    hour_angle = hour * 60 + minute
    minute_angle = minute * 12

    big = (hour_angle - minute_angle) % 720
    small = (minute_angle - hour_angle) % 720

    result = min(big, small)
    # print(time, result)

    return result


def tick():
    times = input().split()
    times = list(map(lambda x: tuple(map(int, x.split(':'))), times))
    times.sort(key=get_angle)

    print(f'{times[2][0]:02d}:{times[2][1]:02d}')


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
