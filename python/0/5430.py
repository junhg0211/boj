from sys import stdin

input = lambda: stdin.readline().rstrip()


def tick():
    functions = input()
    count = int(input())
    numbers = input()[1:-1].split(',')

    start_cut = 0
    last_cut = 0
    start = True
    for function in functions:
        if function == 'D':
            if start_cut + last_cut + 1 > count:
                print('error')
                return

            if start:
                start_cut += 1
                continue

            last_cut += 1
            continue

        start = not start

    if last_cut:
        numbers = numbers[start_cut:-last_cut]
    else:
        numbers = numbers[start_cut:]

    if not start:
        numbers = reversed(numbers)

    print('[' + ','.join(numbers) + ']')


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
