def main():
    a, b = input().split()

    i = 0
    j = 0
    for i, letter in enumerate(a):
        if letter not in b:
            continue
        j = b.index(letter)
        break

    for ii in range(j):
        print("." * i + b[ii] + "." * (len(a) - i - 1))
    print(a)
    for ii in range(j + 1, len(b)):
        print("." * i + b[ii] + "." * (len(a) - i - 1))


if __name__ == "__main__":
    main()
