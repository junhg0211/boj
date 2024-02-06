def main():
    a = input()
    for i in range(5, 10):
        a = a.replace(str(i), str(i-1))
    print(int(a, 9))


if __name__ == '__main__':
    main()
