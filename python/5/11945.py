def main():
    height, _ = map(int, input().split())

    for _ in range(height):
        print(''.join(reversed(input())))


if __name__ == '__main__':
    main()
