from math import inf


cache = [1]


def get_multiplication(from_: int, to: int) -> float:
    a = cache[to]
    b = cache[from_]

    if a == 0 or b == 0:
        return 0
    return a / b


def main():
    count = int(input())

    result = 1
    for i in range(count):
        cache.append(result := result * float(input()))
        if result == 0:
            result = 1

    max_number = -inf
    for i in range(0, count):
        for j in range(i+1, count+1):
            if (value := get_multiplication(i, j)) > max_number:
                max_number = value

    print('%.3f' % max_number)


if __name__ == '__main__':
    main()
