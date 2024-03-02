def f(x):
    result = 0
    now = 1
    while now <= x:
        result += (x+now) // (now<<1) * now

        now <<= 1

    return result


def main():
    a, b = map(int, input().split())

    print(f(b) - f(a-1))


if __name__ == '__main__':
    main()
