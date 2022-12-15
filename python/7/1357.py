def main():
    a, b = map(lambda x: int(x[::-1]), input().split())
    print(str(a + b)[::-1].lstrip('0'))


if __name__ == '__main__':
    main()
