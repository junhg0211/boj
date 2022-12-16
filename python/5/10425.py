fib = [0, 1]


def search(number):
    start = 0
    end = len(fib)

    while True:
        anchor = (start + end) // 2
        if fib[anchor] > number:
            end = anchor
        elif fib[anchor] < number:
            start = anchor
        else:
            return anchor


def tick():
    n = int(input())

    if n == 1:
        print(2)
        return

    last = False
    while fib[-1] < n:
        last = True
        fib.append(fib[-1] + fib[-2])

    if last:
        print(len(fib) - 1)
        return

    print(search(n))


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
