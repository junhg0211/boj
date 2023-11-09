def main():
    string = input()

    substrings = set()
    for length in range(1, len(string)+1):
        for i in range(len(string) - length + 1):
            substring = string[i:i+length]
            substrings.add(substring)

    print(len(substrings))


if __name__ == '__main__':
    main()
