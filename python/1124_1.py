counts = dict()


def factorization(number: int):
    if number in counts:
        return counts[number]

    for i in range(2, number+1):
        if number == i:
            counts[number] = 1
            return 1

        if number % i == 0:
            count = 1 + factorization(number // i)
            counts[number] = count
            return count


def main():
    a, b = map(int, input().split())

    for i in range(b, a-1, -1):
        print(i, factorization(i), len(counts))


if __name__ == '__main__':
    main()
