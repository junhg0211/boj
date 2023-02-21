def move(height, a, b):
    if height == 0:
        return

    another = [1, 2, 3]
    another.remove(a)
    another.remove(b)
    another = another[0]

    move(height-1, a, another)
    print(a, b)
    move(height-1, another, b)


def main():
    count = int(input())

    print(2**count - 1)

    if count > 20:
        return

    move(count, 1, 3)


if __name__ == '__main__':
    main()
