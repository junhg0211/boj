def get_count(length: int, letters: int, been: set = set()):
    if length > letters:
        pass


def main():
    length, count = map(int, input().split())
    input()

    count = get_count(length, count)
    print(count)


if __name__ == '__main__':
    main()
