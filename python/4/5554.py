def main():
    a = int(input()) + int(input()) + int(input()) + int(input())
    print(*divmod(a, 60), sep='\n')


if __name__ == '__main__':
    main()
