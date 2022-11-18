primes = []


def main():
    start = int(input())
    end = int(input())

    min_prime = None
    prime_sum = 0
    for i in range(2, end+1):
        is_prime = True
        for prime in primes:
            if i % prime == 0:
                is_prime = False
                break
        if is_prime:
            primes.append(i)
            if min_prime is None and i >= start:
                min_prime = i
            if min_prime:
                prime_sum += i

    if min_prime is None:
        print('-1')
    else:
        print(prime_sum)
        print(min_prime)


if __name__ == '__main__':
    main()
