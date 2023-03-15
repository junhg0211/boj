def main():
    word = input()
    for i in range(ord('a'), ord('z') + 1):
        print(word.count(chr(i)), end=' ')
    print()


if __name__ == '__main__':
    main()
