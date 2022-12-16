fib = [0, 1]


def update_for(number: int):
    while fib[-1] < number:
        fib.append(fib[-1] + fib[-2])


def get_number(number):
    update_for(number)
    for i, fib_number in enumerate(fib):
        if fib_number == number:
            return number
        if fib_number > number:
            return fib[i-1]


def tick():
    n = int(input())
    numbers = set()
    while n:
        number = get_number(n)
        numbers.add(number)
        n -= number

    print(*map(str, sorted(numbers)), sep=' ')


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
