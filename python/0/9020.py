from sys import stdin

input = stdin.readline

primes = [2, 3, 5, 7]
positions = {2: 0, 3: 1, 5: 2, 7: 3}
prime_count = 4


def is_not_prime(number: int):
    for prime in primes:
        if number % prime == 0:
            return True
    return False


def get_new_prime():
    prime_candidate = primes[-1]
    while is_not_prime(prime_candidate):
        prime_candidate += 2
    return prime_candidate


def calculate_primes_until(number: int):
    global prime_count
    while primes[-1] < number:
        prime = get_new_prime()
        primes.append(prime)
        positions[prime] = prime_count
        prime_count += 1


def tick(number: int):
    index_delta = 0
    pi = None

    i = 0
    while pi is None:
        pi = positions.get(number-i)
        i += 1

    while True:
        for i in range(pi - index_delta + 1):
            candidates = (primes[pi-i-index_delta], primes[pi-i])

            if sum(candidates) < number:
                index_delta += 1
                break

            if sum(candidates) == number:
                print(f'{candidates[0]} {candidates[1]}')
                return


def main():
    count = int(input())
    numbers = [int(input()) for _ in range(count)]

    calculate_primes_until(max(numbers))

    for number in numbers:
        tick(number)


if __name__ == '__main__':
    main()
