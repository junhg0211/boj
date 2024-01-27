def main():
    word = input()

    count = 0
    for letter in word:
        if letter in 'aeiou':
            count += 1

    print(count)


if __name__ == '__main__':
    main()
