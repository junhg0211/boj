def main():
    attendance_count, price_count = map(int, input().split())
    scores = sorted(map(int, input().split()), reverse=True)

    print(scores[price_count-1])


if __name__ == '__main__':
    main()
