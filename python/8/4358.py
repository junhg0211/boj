from sys import stdin

input = stdin.readline


def main():
    frequencies = dict()
    total_count = int()

    while True:
        try:
            name = input().strip()
        except EOFError:
            break
        except KeyboardInterrupt:
            break
        if not name:
            break

        total_count += 1

        if name in frequencies:
            frequencies[name] += 1
        else:
            frequencies[name] = 1

    for name, count in sorted(frequencies.items()):
        print(f'{name} {count/total_count * 100:.4f}')


if __name__ == '__main__':
    main()
