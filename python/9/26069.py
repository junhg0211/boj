def main():
    dancing = {'ChongChong': True}

    for _ in range(int(input())):
        a, b = input().split()

        if dancing.get(a, False):
            dancing[b] = True
        if dancing.get(b, False):
            dancing[a] = True

    print(len(dancing))


if __name__ == '__main__':
    main()
