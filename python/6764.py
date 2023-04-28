def main():
    constanting = True
    constant = None

    increasing = True
    increase = None

    decreasing = True
    decrease = None

    for _ in range(4):
        number = int(input())

        if constant is None:
            constant = number
        elif number != constant:
            constanting = False

        if increase is None:
            increase = number
        elif number <= increase:
            increasing = False

        if decrease is None:
            decrease = number
        elif number >= decrease:
            decreasing = False

    if constanting:
        print('Constant Depth')
    elif increasing:
        print('Fish Rising')
    elif decreasing:
        print('Fish Diving')


if __name__ == '__main__':
    main()
