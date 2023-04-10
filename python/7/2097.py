from math import sqrt, ceil


def main():
    count = int(input())

    if count <= 2:
        print(4)
        return

    width = ceil(sqrt(count))
    height = ceil(count / width)

    print((width + height - 2) * 2)


if __name__ == '__main__':
    main()
