from sys import stdin

input = stdin.readline


def main():
    string = input().rstrip()
    explosive = list(input().rstrip())

    stack = list()
    for letter in string:
        stack.append(letter)

        while stack[-len(explosive):] == explosive:
            for _ in range(len(explosive)):
                stack.pop()

    if stack:
        print(''.join(stack))
    else:
        print('FRULA')


if __name__ == '__main__':
    main()
