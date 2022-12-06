def main():
    for _ in range(int(input())):
        string = list(input())
        stack = 0
        valid = True
        while string:
            if string.pop() == ')':
                stack += 1
            else:
                stack -= 1

            if stack < 0:
                valid = False
                break

        if valid and stack == 0:
            print('YES')
        else:
            print('NO')


if __name__ == '__main__':
    main()
