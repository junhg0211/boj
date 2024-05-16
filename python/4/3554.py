def main():
    input()
    numbers = list(map(int, input().split()))

    work_count = int(input())
    for _ in range(work_count):
        t, l, r = map(int, input().split())
        l -= 1
        r -= 1

        if t == 1:
            for i in range(l, r + 1):
                numbers[i] = numbers[i] ** 2 % 2010
            continue

        sum_ = 0
        for i in range(l, r + 1):
            sum_ += numbers[i]
        print(sum_)


if __name__ == "__main__":
    main()
