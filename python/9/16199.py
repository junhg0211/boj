from datetime import date


def main():
    birthday = date(*map(int, input().split()))
    today = date(*map(int, input().split()))

    yeon = today.year - birthday.year
    today = date(today.year - yeon, today.month, today.day)

    if birthday <= today:
        print(yeon)
    else:
        print(yeon-1)

    print(yeon+1)
    print(yeon)


if __name__ == '__main__':
    main()
