from collections import deque


def main():
    input()

    line = deque(map(int, input().split()))
    stack = list()

    now = 1
    while line or stack:
        # print(line, stack)

        if line:
            this = line[0]

            if this == now:
                line.popleft()
                now += 1
                continue

            if stack and stack[-1] == now:
                stack.pop()
                now += 1
                continue

            stack.append(line.popleft())
            continue

        elif stack:
            if now == stack[-1]:
                stack.pop()
                now += 1
                continue

            break

    if line or stack:
        print('Sad')
    else:
        print('Nice')


if __name__ == '__main__':
    main()
