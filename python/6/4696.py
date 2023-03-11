def main():
    while a := float(input()):
        print(format(1 + a + a**2 + a**3 + a**4, '.2f'))


if __name__ == '__main__':
    main()
