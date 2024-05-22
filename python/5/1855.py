def main():
    yeol_count = int(input())
    string = input()

    for i in range(len(string)):
        x, y = divmod(i, len(string) // yeol_count)
        if y % 2 == 1:
            x = yeol_count - x - 1
        print(string[y * yeol_count + x], end="")
    print()


if __name__ == "__main__":
    main()
