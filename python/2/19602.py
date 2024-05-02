def main():
    a = int(input())
    b = int(input())
    c = int(input())

    score = a + b * 2 + c * 3

    if score >= 10:
        print("happy")
    else:
        print("sad")


if __name__ == "__main__":
    main()
