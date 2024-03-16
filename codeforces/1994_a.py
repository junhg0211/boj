def tick():
    a, b = map(int, input().split())

    if a - 1 <= b:
        print("1")
    else:
        print(a)


def main():
    testcase_count = int(input())

    for _ in range(testcase_count):
        tick()


if __name__ == "__main__":
    main()
