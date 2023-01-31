primes = [2]
primes_set = {2}


def is_prime(number: int):
    for candidate in range(primes[-1], number+1):
        prime = True
        sqrt = number**0.5
        for n in primes:
            if n > sqrt:
                break
            if candidate % n == 0:
                prime = False
                break
        if prime:
            primes.append(candidate)
            primes_set.add(candidate)

    return number in primes_set


def get_sequence(numbers: int, length: int, start: int = 0):
    if length == 1:
        for i in range(start, numbers):
            yield (i,)
        return

    for i in range(start, numbers):
        for sequence in get_sequence(numbers, length-1, i+1):
            yield (i,) + sequence


def main():
    m, n = map(int, input().split())
    weights = list(map(int, input().split()))

    primes = set()
    for sequence in get_sequence(m, n):
        sum_ = sum(map(lambda x: weights[x], sequence))
        if is_prime(sum_):
            primes.add(sum_)

    if primes:
        print(' '.join(map(str, sorted(primes))))
    else:
        print('-1')


if __name__ == '__main__':
    main()
