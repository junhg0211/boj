def main():
    a, b = map(int, input().split())
    print(min((a-1) + a*(b-1), (b-1) + b*(a-1)))


if __name__ == '__main__':
    main()
