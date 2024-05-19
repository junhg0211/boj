def main():
    i = 1
    while True:
        o, w = map(int, input().split())

        if o == w == 0:
            break

        while True:
            do, n = input().split()
            n = int(n)

            if do == "#":
                break

            if do == "F" and w > 0:
                w += n
            else:
                w -= n

        print(f"{i} ", end="")
        if o < 2 * w < 4 * o:
            print(":-)")
        elif w <= 0:
            print("RIP")
        else:
            print(":-(")

        i += 1


if __name__ == "__main__":
    main()
