fib = [0, 1]


def tick(case_number):
    n, divisor = map(int, input().split())

    while len(fib) <= n:
        fib.append(fib[-1] + fib[-2])

    result = fib[n] % divisor
    print(f'Case #{case_number}: {result}')


def main():
    for i in range(1, int(input())+1):
        tick(i)


if __name__ == '__main__':
    main()
