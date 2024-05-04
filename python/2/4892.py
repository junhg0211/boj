def main():
    i = 1

    while True:
        number = int(input())

        if number == 0:
            break

        print(f"{i}. ", end="")

        if number * 3 % 2 == 0:
            print("even", end=" ")
            n2 = 3 * number // 2
        else:
            print("odd", end=" ")
            n2 = (3 * number + 1) // 2

        n3 = 3 * n2

        n4 = n3 // 9

        print(n4)

        i += 1


if __name__ == "__main__":
    main()
