def main():
    a, b = map(int, input().split())
    max_ = max(a, b) * 2

    if max_ == 0:
        print('Not a moose')
    elif a == b:
        print(f'Even {max_}')
    else:
        print(f'Odd {max_}')


if __name__ == '__main__':
    main()
