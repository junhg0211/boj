def main():
    numbers = [int(input()) for _ in range(5)]
    print(sum(numbers) // 5)
    print(sorted(numbers)[2])


if __name__ == '__main__':
    main()
