from itertools import permutations


def main():
    _ = int(input())
    numbers = map(int, input().split())

    result = 0
    for permutation in permutations(numbers):
        now = 0
        for i in range(len(permutation) - 1):
            now += abs(permutation[i] - permutation[i - 1])
        result = max(now, result)

    print(result)


if __name__ == "__main__":
    main()
