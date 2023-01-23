def main():
    target, length = map(int, input().split())

    i = length
    while True:
        if i % 2 == 1:
            if target % i != 0:
                i += 1
                continue

            start = target // i - i // 2

            if start < 0:
                print('-1')
                return

            for j in range(start, start+i):
                print(j, end=' ')
            print()
            return

        if target / i % 1 == 0.5:
            start = target // i - (i//2 - 1)

            if start < 0:
                print('-1')
                return

            for j in range(start, start+i):
                print(j, end=' ')
            print()
            return

        i += 1

        if i > 100:
            print('-1')
            return


if __name__ == '__main__':
    main()
