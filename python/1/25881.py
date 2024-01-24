def main():
    a, b = map(int, input().split())
    count = int(input())

    for _ in range(count):
        consumption = int(input())

        if consumption >= 1000:
            result = (consumption - 1000) * b + 1000 * a
        else:
            result = consumption * a

        print(consumption, result)


if __name__ == '__main__':
    main()
