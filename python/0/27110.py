def main():
    limit = int(input())
    print(sum(map(lambda x: min(limit, int(x)), input().split())))


if __name__ == "__main__":
    main()
