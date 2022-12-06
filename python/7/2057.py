def main():
    factorials = [1]
    number = int(input())

    if number == 0:
        print('NO')
        return

    for i in range(1, 21):
        factorials.append(factorials[-1] * i)

    for factorial in reversed(factorials):
        if number >= factorial:
            number -= factorial

    if number:
        print('NO')
    else:
        print('YES')


if __name__ == '__main__':
    main()
