def main():
    number = int(input())

    count = 0
    for i in range(100):
        for j in range(100):
            if i + j == number:
                count += 1

    print(count)


if __name__ == "__main__":
    main()
