def main():
    i = 1
    while True:
        a, b = map(int, input().split())

        if a == b == 0:
            break

        print(f"Hole #{i}")
        if b == 1:
            print("Hole-in-one.")
        elif a - b >= 3:
            print("Double eagle.")
        elif a - b >= 2:
            print("Eagle.")
        elif a - b >= 1:
            print("Birdie.")
        elif a - b >= 0:
            print("Par.")
        elif a - b >= -1:
            print("Bogey.")
        else:
            print("Double Bogey.")
        print()

        i += 1


if __name__ == "__main__":
    main()
