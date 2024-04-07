def main():
    message = input()

    for letter in message:
        if letter == "a":
            print("4", end="")
            continue
        if letter == "e":
            print("3", end="")
            continue
        if letter == "i":
            print("1", end="")
            continue
        if letter == "o":
            print("0", end="")
            continue
        if letter == "s":
            print("5", end="")
            continue

        print(letter, end="")

    print()


if __name__ == "__main__":
    main()
