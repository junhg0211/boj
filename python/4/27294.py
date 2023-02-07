def main():
    time, sool = map(int, input().split())

    if sool or not (12 <= time <= 16):
        print('280')
    else:
        print('320')


if __name__ == '__main__':
    main()
