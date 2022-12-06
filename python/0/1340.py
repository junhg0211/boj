names = 'January, February, March, April, May, June, July, August, September, October, November, December'.split(', ')
days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]


def parse_datetime(string: str):
    month, day, year, time = string.split()
    month = names.index(month)
    day = int(day[:-1]) - 1
    year = int(year)
    hour, minute = map(int, time.split(':'))
    return year, month, day, hour, minute


def main():
    year, month, day, hour, minute = parse_datetime(input())
    if year % 4 == 0 and year % 100 != 0 or year % 400 == 0:
        days[1] = 29
    print((sum(days[:month]) + day + (hour + (minute/60)) / 24) / sum(days) * 100)


if __name__ == '__main__':
    main()
