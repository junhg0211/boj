from math import inf


def gcd(a: int, b: int):
    if b > a:
        return gcd(b, a)

    if c := a % b:
        return gcd(b, c)

    return b


def lcm(a: int, b: int):
    return a * b // gcd(a, b)


def main():
    numbers = tuple(map(int, input().split()))

    minimum = inf
    if (a := lcm(lcm(numbers[0], numbers[1]), numbers[2])) < minimum:
        minimum = a
    if (a := lcm(lcm(numbers[0], numbers[1]), numbers[3])) < minimum:
        minimum = a
    if (a := lcm(lcm(numbers[0], numbers[1]), numbers[4])) < minimum:
        minimum = a
    if (a := lcm(lcm(numbers[0], numbers[2]), numbers[3])) < minimum:
        minimum = a
    if (a := lcm(lcm(numbers[0], numbers[2]), numbers[4])) < minimum:
        minimum = a
    if (a := lcm(lcm(numbers[0], numbers[3]), numbers[4])) < minimum:
        minimum = a
    if (a := lcm(lcm(numbers[1], numbers[2]), numbers[3])) < minimum:
        minimum = a
    if (a := lcm(lcm(numbers[1], numbers[2]), numbers[4])) < minimum:
        minimum = a
    if (a := lcm(lcm(numbers[1], numbers[3]), numbers[4])) < minimum:
        minimum = a
    if (a := lcm(lcm(numbers[2], numbers[3]), numbers[4])) < minimum:
        minimum = a

    print(minimum)


if __name__ == '__main__':
    main()
