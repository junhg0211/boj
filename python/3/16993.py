def main():
    a, b = map(int, input().split())
    c, d = map(int, input().split())

    slice_ = a/b
    whole = c*c*3.14159265358979/d

    if slice_ > whole:
        print('Slice of pizza')
    else:
        print('Whole pizza')


if __name__ == '__main__':
    main()
