def main():
    n, u, l = map(int, input().split())

    a = n >= 1000
    b = u >= 8000 or l >= 260

    if a and b:
        print("Very Good")
    elif a:
        print("Good")
    else:
        print("Bad")


if __name__ == "__main__":
    main()
