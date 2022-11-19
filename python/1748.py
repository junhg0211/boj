from math import log10

def main():
    number = int(input())

    if number < 10:
        print(number)
        return

    log = int(log10(number))
    result = 9

    for i in range(1, log):
        result += (10**(i+1) - 10**i) * (i + 1)

    result += (number - 10**log + 1) * (log + 1)

    print(result)


if __name__ == '__main__':
    main()
