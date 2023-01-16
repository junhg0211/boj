def main():
    now = 0
    future = True

    for _ in range(int(input())):
        what, time = input().split()
        time = int(time) - 1

        print(now+1, end=' ')
        if time == now and what != 'HOURGLASS':
            print('YES')
        else:
            print('NO')

        if what == 'HOURGLASS' and time != now:
            future = not future

        if future:
            now += 1
        else:
            now -= 1
        now %= 12


if __name__ == '__main__':
    main()
