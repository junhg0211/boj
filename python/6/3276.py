def main():
    count = int(input())

    w, h = 1, 1
    while w * h < count:
        if w < h:
            w += 1
        else:
            h += 1

    print(w, h)


if __name__ == "__main__":
    main()
