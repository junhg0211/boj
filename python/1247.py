def tick():
    count = int(input())

    sum_ = 0
    for _ in range(count):
        sum_ += int(input())

    if sum_ > 0:
        print('+')
    elif sum_ < 0:
        print('-')
    else:
        print('0')


def main():
    for _ in range(3):
        tick()


if __name__ == '__main__':
    main()
