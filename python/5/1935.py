def main():
    count = int(input())
    equation = input()
    numbers = [int(input()) for _ in range(count)]

    stack = list()
    for letter in equation:
        if 'A' <= letter:
            stack.append(numbers[ord(letter) - ord('A')])
        elif letter == '+':
            b = stack.pop()
            a = stack.pop()
            stack.append(a + b)
        elif letter == '-':
            b = stack.pop()
            a = stack.pop()
            stack.append(a - b)
        elif letter == '*':
            b = stack.pop()
            a = stack.pop()
            stack.append(a * b)
        elif letter == '/':
            b = stack.pop()
            a = stack.pop()
            stack.append(a / b)
        elif letter == '%':
            b = stack.pop()
            a = stack.pop()
            stack.append(a % b)

    print(format(stack[0], '.2f'))


if __name__ == '__main__':
    main()
