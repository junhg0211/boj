def is_yoon(year: int) -> bool:
    return year % 4 == 0 and year % 100 != 0 or year % 400 == 0


DAYS = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]


def get_year_day(month: int, day: int, yoon: bool):
    result = day
    if yoon and month >= 3:
        result += 1

    for i in range(month):
        result += DAYS[i]

    return result - 1


def normal(today: tuple, ends: tuple):
    days = 0
    for i in range(today[0], ends[0]):
        days += 365 + int(is_yoon(i))

    days -= get_year_day(today[1], today[2], is_yoon(today[0]))
    days += get_year_day(ends[1], ends[2], is_yoon(ends[0]))

    print('D-', days, sep='')


def main():
    today = tuple(map(int, input().split()))
    ends = tuple(map(int, input().split()))

    if ends[0] > today[0] + 1000:
        print('gg')
    elif ends[0] == today[0] + 1000:
        if ends[1] > today[1]:
            print('gg')
        elif ends[1] == today[1]:
            if ends[2] >= today[2]:
                print('gg')
            else:
                normal(today, ends)
        else:
            normal(today, ends)
    else:
        normal(today, ends)


if __name__ == '__main__':
    main()

