def normal(number, base):
    sign = number < 0
    number = abs(number)

    result = list()

    while abs(number) > 0:
        result.append(str(number % base))
        number //= base

    print("-" if sign else "", "".join(reversed(result)), sep="")


def positive(number, base):
    base = -base

    result = list()
    carry = 0
    while number > 0 or carry:
        if carry:
            number += carry
            carry = 0

        one = number % base
        number //= base
        result.append(str(one))

        ten = number % base
        number //= base
        if ten == 0:
            result.append(str(0))
        else:
            carry = 1
            result.append(str(base - ten))

    while result[-1] == "0":
        result.pop()

    print("".join(reversed(result)))


def negative(number, base):
    number, base = -number, -base

    result = list()
    while number > 0:
        one = number % base
        number //= base
        if one == 0:
            result.append(str(0))
        else:
            result.append(str(base - one))
            number += 1

        ten = number % base
        number //= base
        result.append(str(ten))

    while result[-1] == "0":
        result.pop()

    print("".join(reversed(result)))


def negative_base(number, base):
    if number > 0:
        positive(number, base)

    else:
        negative(number, base)


def main():
    number, base = map(int, input().split())

    if number == 0:
        print("0")
        return

    if base > 0:
        normal(number, base)
    else:
        negative_base(number, base)


if __name__ == "__main__":
    main()
