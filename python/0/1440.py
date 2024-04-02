from itertools import permutations


def main():
    result = 0
    numbers = list(map(int, input().split(":")))
    for combination in permutations(numbers):
        hour, minute, second = combination
        if 1 <= hour <= 12 and 0 <= minute <= 59 and 0 <= second <= 59:
            result += 1

    print(result)


if __name__ == "__main__":
    main()
