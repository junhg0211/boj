from sys import stdin

input = stdin.readline

numbers = '0', '1', '2486', '3971', '46', '5', '6', '7931', '8426', '91'

def tick():
    a, b = map(int, input().split())

    value = numbers[a%10][(b-1)%len(numbers[a%10])]
    if value == '0':
        print('10')
    else:
        print(value)


def main():
    for _ in range(int(input())):
        tick()


if __name__ == '__main__':
    main()
