from sys import stdout

print = lambda x: stdout.write(x + '\n')


def main():
    n = int(input())

    if n == 1:
        print(str(n))
        print('1 1')
        return

    if n == 2:
        print(str(n))
        print('1 1\n2 1')
        return

    print(str((n-1) * 2))

    for i in range(n-1):
        print(f'{i+1} 1')
        print(f'{i+1} {n}')


if __name__ == '__main__':
    main()
