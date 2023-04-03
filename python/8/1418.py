primes = list()


def is_prime(number: int):
    for prime in primes:
        if number % prime == 0:
            return False
    return True


def update_primes(until: int):
    for i in range(max(primes, default=1)+1, until+1):
        if is_prime(i):
            primes.append(i)


def get_big_divisor(number: int):
    result = 0
    for prime in primes:
        if prime > number:
            break
        if number % prime == 0:
            result = prime

    return result


def main():
    n, k = int(input()), int(input())

    update_primes(n)

    count = 0
    for i in range(1, n+1):
        if get_big_divisor(i) <= k:
            count += 1

    print(count)


if __name__ == '__main__':
    main()
