def main():
    count = 0
    for _ in range(int(input())):
        if int(input()[2:]) <= 90:
            count += 1

    print(count)


if __name__ == '__main__':
    main()
