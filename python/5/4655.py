def main():
    while True:
        number = float(input())

        if number == 0.0:
            break

        under = 2
        sum_ = 0.0

        while sum_ + 1 / under < number:
            sum_ += 1 / under
            under += 1

        print(f"{under - 1} card(s)")


if __name__ == "__main__":
    main()
