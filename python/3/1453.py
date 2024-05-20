def main():
    input()

    sits = set()
    count = 0
    for number in map(int, input().split()):
        if number in sits:
            count += 1
        sits.add(number)

    print(count)


if __name__ == "__main__":
    main()
