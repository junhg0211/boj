def jarisu_hap(number):
    result = 0
    while number > 0:
        result += number % 10
        number //= 10

    return result


def is_perfect(number):
    return number == 4 or number > 5


def jjaksu_soinsu(number):
    if number == 1:
        return False

    soinsus = dict()

    i = 2
    while number > 1:
        while number % i == 0:
            number //= i
            soinsus[i] = soinsus.get(i, 0) + 1

        i += 1

    return len(soinsus) % 2 == 0


def main():
    number = int(input())

    imyeonsu = is_perfect(number) and jarisu_hap(number) % 2 == 1
    imhyeonsu = number == 4 or number == 2 or jjaksu_soinsu(number)

    if imyeonsu and imhyeonsu:
        print(4)
    elif not imyeonsu and not imhyeonsu:
        print(3)
    elif imyeonsu:
        print(1)
    else:
        print(2)


if __name__ == "__main__":
    main()
