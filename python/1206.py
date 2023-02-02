from collections import deque


def gcd(a: int, b: int):
    if a < b:
        return gcd(b, a)

    while True:
        a, b = b, a % b
        if b == 0:
            return a


def main():
    count = int(input())
    results = sorted(round(float(input()) * 1000) for _ in range(count))

    if not results:
        print('1')
        return

    differences = deque(map(lambda x: int(x), results))

    for i in range(count):
        for j in range(i+1, count):
            differences.append(int(results[j] - results[i]))

    while len(differences) > 1:
        print(differences)
        a = differences.popleft()
        b = differences.popleft()
        differences.append(gcd(a, b))

    print(round(1/differences[0]*1000))


if __name__ == '__main__':
    main()
