from copy import copy


def primify(number: int) -> list[int]:
    result = list()

    i = 2
    while number > 1:
        while number % i == 0:
            result.append(i)
            number //= i
        i += 1

    return result


def main():
    for _ in range(int(input())):
        a, b = map(int, input().split())

        if a == 1:
            print('yes' if b == 1 else 'no')
            break

        primified = primify(a)
        limit = 1 << (len(primified) - 1)

        yes = False

        for i in range(0, limit):
            divisors = copy(primified)
            j = len(divisors) - 1
            while i > 0 and j > 0:
                if i & 1:
                    divisors[j-1:j+1] = [divisors[j-1] * divisors[j]]
                i >>= 1
                j -= 1

            if sum(divisors) == b:
                yes = True
                break

        print('yes' if yes else 'no')


if __name__ == '__main__':
    main()
