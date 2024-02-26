def main():
    for _ in range(int(input())):
        number = int(input())

        pairs = list()
        for i in range(1, 12):
            for j in range(i+1, 12):
                if i + j == number:
                    pairs.append(f'{i} {j}')
                    break

        pairs = ', '.join(pairs)
        print(f'Pairs for {number}: {pairs}')


if __name__ == '__main__':
    main()
