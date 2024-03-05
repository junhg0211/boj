from sys import stdout


def main():
    n = int(input())

    start = 2
    end = n

    while start < end:
        # print(start, end)
        mid = (start + end) // 2

        print("?", mid)
        stdout.flush()

        answer = input() == "1"

        if answer:
            start = mid + 1
        else:
            end = mid

    print(end)


if __name__ == "__main__":
    main()
