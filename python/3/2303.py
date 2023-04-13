from sys import stdin

input = stdin.readline


def get_max_one(numbers):
    result = 0
    for i in range(5):
        for j in range(i+1, 5):
            for k in range(j+1, 5):
                result = max((numbers[i] + numbers[j] + numbers[k]) % 10, result)

    return result


def main():
    count = int(input())

    max_one = -1
    max_number = -1

    for i in range(count):
        numbers = tuple(map(int, input().split()))
        one = get_max_one(numbers)

        if one >= max_one:
            max_one = one
            max_number = i+1

    print(max_number)


if __name__ == '__main__':
    main()
