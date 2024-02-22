def main():
    for i in range(1, int(input()) + 1):
        if i % 77 == 0:
            print('Wiwat!')
        elif i % 11 == 0:
            print('Super!')
        elif i % 7 == 0:
            print('Hurra!')
        else:
            print(i)


if __name__ == '__main__':
    main()
