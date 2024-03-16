from decimal import Decimal


def main():
    count, length, width, height = map(Decimal, input().split())

    start = Decimal('0')
    end = max(length, width, height)

    for _ in range(10000):
        mid = (start + end) / 2

        wow = (length // mid) * (width // mid) * (height // mid)
        # print(start, end, wow)

        if wow < count:
            end = mid
        else:
            start = mid

    print((start + end) / 2)


if __name__ == "__main__":
    main()
