from math import inf


def main():
    sum_ = 0
    min_ = inf
    for number in (int(input()) for _ in range(7)):
        if number % 2 != 0:
            sum_ += number
            min_ = min(min_, number)

    if min_ == inf:
        print('-1')
        return

    print(sum_, min_, sep='\n')


if __name__ == '__main__':
    main()
