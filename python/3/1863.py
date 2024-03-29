def tick(stack, height):
    delta = 0

    while stack and stack[-1] >= height:
        if stack[-1] > height:
            delta += 1
        stack.pop()

    stack.append(height)

    return delta


def main():
    change_count = int(input())

    stack: list[int] = list()
    result = 0
    for _ in range(change_count):
        _, height = map(int, input().split())

        result += tick(stack, height)

    result += tick(stack, 0)

    print(result)


if __name__ == "__main__":
    main()
