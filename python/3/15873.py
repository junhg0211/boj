def main():
    a = input()

    if len(a) == 2:
        print(int(a[0]) + int(a[1]))
    elif len(a) == 3:
        if a[1] == '0':
            print(10 + int(a[2]))
        else:
            print(int(a[0]) + 10)
    else:
        print(20)


if __name__ == '__main__':
    main()
