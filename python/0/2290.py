def main():
    length, value = input().split()
    length = int(length)

    r1 = [
        ' ' + '-'*length + ' ',
        ' ' + ' '*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + ' '*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' '
    ]
    r2 = [
        '|' + ' '*length + '|',
        ' ' + ' '*length + '|',
        ' ' + ' '*length + '|',
        ' ' + ' '*length + '|',
        '|' + ' '*length + '|',
        '|' + ' '*length + ' ',
        '|' + ' '*length + ' ',
        ' ' + ' '*length + '|',
        '|' + ' '*length + '|',
        '|' + ' '*length + '|'
    ]
    r3 = [
        ' ' + ' '*length + ' ',
        ' ' + ' '*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + ' '*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' '
    ]
    r4 = [
        '|' + ' '*length + '|',
        ' ' + ' '*length + '|',
        '|' + ' '*length + ' ',
        ' ' + ' '*length + '|',
        ' ' + ' '*length + '|',
        ' ' + ' '*length + '|',
        '|' + ' '*length + '|',
        ' ' + ' '*length + '|',
        '|' + ' '*length + '|',
        ' ' + ' '*length + '|'
    ]
    r5 = [
        ' ' + '-'*length + ' ',
        ' ' + ' '*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + ' '*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + ' '*length + ' ',
        ' ' + '-'*length + ' ',
        ' ' + '-'*length + ' '
    ]

    for letter in value:
        print(r1[int(letter)], end=' ')
    print()
    for _ in range(length):
        for letter in value:
            print(r2[int(letter)], end=' ')
        print()
    for letter in value:
        print(r3[int(letter)], end=' ')
    print()
    for _ in range(length):
        for letter in value:
            print(r4[int(letter)], end=' ')
        print()
    for letter in value:
        print(r5[int(letter)], end=' ')
    print()


if __name__ == '__main__':
    main()
