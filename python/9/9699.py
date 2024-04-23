def main():
    count = int(input())

    for i in range(1, count + 1):
        print(f"Case #{i}: {max(map(int, input().split()))}")


if __name__ == "__main__":
    main()
