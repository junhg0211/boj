def main():
    month, day = int(input()), int(input())

    if month == 2 and day == 18:
        print('Special')
    elif month == 1 or month == 2 and day < 18:
        print('Before')
    else:
        print('After')


if __name__ == "__main__":
    main()
