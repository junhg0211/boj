def main():
    days = int(input())

    a = int(input())
    b = int(input())
    a /= int(input())
    b /= int(input())

    print(min(int(days - a), int(days - b)))


if __name__ == '__main__':
    main()
