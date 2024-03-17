def tick():
    numbers = map(int, input().split())

    count = next(numbers)

    counts = dict()
    for number in numbers:
        counts[number] = counts.get(number, 0) + 1

    byeongsa, byeongsa_count = max(counts.items(), key=lambda x: x[1])

    if byeongsa_count > count // 2:
        print(byeongsa)
    else:
        print("SYJKGW")


def main():
    testcase_count = int(input())

    for _ in range(testcase_count):
        tick()


if __name__ == "__main__":
    main()
