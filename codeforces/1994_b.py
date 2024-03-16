from itertools import combinations


def xor(*numbers):
    result = 0

    for number in numbers:
        result ^= number

    return result


def tick():
    one_map = dict()
    two_map = dict()

    n, k = map(int, input().split())
    numbers = list(map(int, input().split()))

    for combination1 in combinations(numbers[:n], k * 2):
        for combination2 in combinations(numbers[n:], k * 2):
            one_xor = xor(*combination1)
            two_xor = xor(*combination2)

            one_map[one_xor] = combination1
            two_map[two_xor] = combination2

            if one_xor == two_xor:
                print(" ".join(map(str, combination1)))
                print(" ".join(map(str, combination2)))
                return

            if one_xor in two_map:
                print(" ".join(map(str, combination1)))
                print(" ".join(map(str, two_map[one_xor])))
                return

            if two_xor in one_map:
                print(" ".join(map(str, one_map[two_xor])))
                print(" ".join(map(str, combination2)))
                return


def main():
    testcase_count = int(input())

    for _ in range(testcase_count):
        tick()


if __name__ == "__main__":
    main()
