def main():
    jinho = input()

    count = 0
    for _ in range(int(input())):
        mbti = input()

        if mbti == jinho:
            count += 1

    print(count)


if __name__ == '__main__':
    main()
