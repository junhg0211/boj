def gcd(a, b):
    if a > b:
        return gcd(b, a)

    c = True
    while c:
        c = b % a
        b = a
        a = c

        if a == 0:
            return b


def main():
    input()
    numbers = map(int, input().split())

    original_size = next(numbers)
    for number in numbers:
        divisor = gcd(number, original_size)
        print(f'{original_size//divisor}/{number//divisor}')

if __name__ == '__main__':
    main()

