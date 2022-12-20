def main():
    command = input()

    nops = 0
    for i, letter in enumerate(command):
        if letter.isupper():
            nops += -(i+nops) % 4

    print(nops)


if __name__ == '__main__':
    main()
