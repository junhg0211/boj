def main():
    n, k, a, b = map(int, input().split())

    stairs = a * (n - 1)
    lift = (k + n - 2) * b

    print(lift, stairs)


if __name__ == "__main__":
    main()
