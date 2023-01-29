from math import sin, pi


def function(x, a, b):
    return a*x + b*sin(x)


def main():
    a, b, c = map(int, input().split())

    start = c/a - pi
    end = c/a + pi

    while end-start > 0.0000001:
        middle_value = (start + end) / 2
        middle_function = function(middle_value, a, b)

        if middle_function > c:
            end = middle_value
        else:
            start = middle_value

        # print(start, middle_value, end)

    print(middle_value)


if __name__ == '__main__':
    main()
