def main():
    start, end = map(int, input().split())

    result = 0
    counts = [0, 0]
    primes = list()
    for i in range(2, end+1):
        updated = False
        for prime in primes:
            if i % prime == 0:
                count = counts[prime] + counts[i//prime]
                counts.append(count)
                updated = True
                break
        if not updated:
            primes.append(i)
            counts.append(1)

    print(sum(1 for _ in filter(lambda x: counts[x] == 1, counts[start:])))


if __name__ == '__main__':
    main()
