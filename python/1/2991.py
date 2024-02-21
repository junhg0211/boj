def main():
    a, b, c, d = map(int, input().split())

    for time in map(int, input().split()):
        time -= 1

        result = 0

        if time % (a + b) < a:
            result += 1

        if time % (c + d) < c:
            result += 1

        print(result)


if __name__ == '__main__':
    main()
