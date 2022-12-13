def f(n: str) -> str:
    return str(sum(map(int, n)))


def main():
    number = input()

    i = 0
    while len(number) > 1:
        number = f(number)
        i += 1

    print(i)

    if number in '369':
        print('YES')
        return

    print('NO')


if __name__ == '__main__':
    main()
