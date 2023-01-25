primes = list()
primes_set = None


def update_prime(number: int):
    root = number ** 0.5
    for i in primes:
        if i > root:
            break
        if number % i == 0:
            return

    primes.append(number)


counts = dict()


def factorization(number: int):
    if number in primes_set:
        return 1

    if number in counts:
        return counts[number]

    original_number = number
    prime_count = int()
    for prime in primes:
        while number % prime == 0:
            prime_count += 1
            number //= prime

            if number in counts:
                value = prime_count + counts[number]
                counts[original_number] = value
                return value

        if number == 1:
            break

    counts[original_number] = prime_count
    return prime_count


def main():
    global primes_set

    a, b = map(int, input().split())

    for i in range(2, b+1):
        update_prime(i)

    primes_set = set(primes)

    result = 0
    for i in range(2, b+1):
        # print(i, len(counts))
        count = factorization(i)
        if i >= a and count in primes:
            result += 1

    print(result)


if __name__ == '__main__':
    main()
