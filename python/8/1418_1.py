primes = list()


def is_prime(number: int):
    for prime in primes:
        if number % prime == 0:
            return False
    return True


def update_primes(until: int):
    for i in range(2, until):
        if is_prime(i):
            primes.append(i)


def main():
    n, k = int(input()), int(input())

    update_primes(n//2 + 1)

    count = 0
    for i in range(1, n+1):
        biggest = 0
        for prime in primes:
            while i % prime == 0:
                i //= prime
                biggest = prime
            if i == 1:
                break
        if biggest == 0:
            biggest = i

        # print(i, biggest)

        if biggest <= k:
            count += 1

    print(count)


if __name__ == '__main__':
    main()
