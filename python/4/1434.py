def main():
    chest_count, book_count = map(int, input().split())

    chest_sizes = list(map(int, input().split()))
    book_sizes = list(map(int, input().split()))

    now_chest = 0
    i = 0
    while i < book_count:
        book = book_sizes[i]

        if book > chest_sizes[now_chest]:
            now_chest += 1
        else:
            chest_sizes[now_chest] -= book
            i += 1

    print(sum(chest_sizes))


if __name__ == "__main__":
    main()
