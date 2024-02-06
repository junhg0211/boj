from sys import stdin

input = stdin.readline


def combinations(length: int):
    result = list()
    for i in range(2**length):
        result.clear()
        j = 0
        while i:
            if i & 1:
                result.append(j)
            i >>= 1
            j += 1
        yield result


def main():
    count, target_sum = map(int, input().split())
    numbers = list(map(int, input().split()))

    result = 0
    for combination in combinations(count):
        if not combination:
            continue
        if sum(map(lambda x: numbers[x], combination)) == target_sum:
            result += 1

    print(result)


if __name__ == '__main__':
    main()
