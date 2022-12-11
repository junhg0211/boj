cache = dict()


def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1

    if n in cache:
        return cache[n]

    cache[n] = fib(n-1) + fib(n-2)
    return cache[n]


def main():
    print(fib(170))


if __name__ == '__main__':
    main()

