def main():
    polynomial = input()

    if 'x' not in polynomial:
        print('0')
        return

    a, _ = polynomial.split('x')

    if not a:
        print('1')
    elif a == '-':
        print('-1')
    else:
        print(a)


if __name__ == '__main__':
    main()
