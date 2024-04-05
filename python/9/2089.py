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
    number = int(input())
    base = -2

    if number == 0:
        print("0")
        return

    negative_base(number, base)


if __name__ == "__main__":
    main()
