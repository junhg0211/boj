def main():
    w1, h1 = int(input()), int(input())
    w2, h2 = int(input()), int(input())

    print(2 * (max(w1, w2) + 1 + h1 + h2 + 1))


if __name__ == '__main__':
    main()
