def main():
    height, width, limit = map(int, input().split())

    if width + height - 1 > limit:
        print("NO")
        return

    print("YES")

    for i in range(height):
        for j in range(width):
            print(i + j + 1, end=" ")
        print()


if __name__ == "__main__":
    main()
