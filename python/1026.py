def main():
    n = int(input())

    a_numbers = sorted(map(int, input().split()))
    b_numbers = sorted(map(int, input().split()), reverse=True)

    print(sum(map(lambda i: a_numbers[i] * b_numbers[i], range(n))))


if __name__ == '__main__':
    main()
