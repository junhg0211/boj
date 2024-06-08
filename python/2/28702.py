def is_number(string: str) -> bool:
    try:
        int(string)
        return True
    except ValueError:
        return False


def main():
    a = input()
    b = input()
    c = input()

    a_number = -1
    if is_number(a):
        a_number = int(a)
    elif is_number(b):
        a_number = int(b) - 1
    elif is_number(c):
        a_number = int(c) - 2

    number = a_number + 3

    if number % 3 == 0 and number % 5 == 0:
        print("FizzBuzz")
    elif number % 3 == 0:
        print("Fizz")
    elif number % 5 == 0:
        print("Buzz")
    else:
        print(number)


if __name__ == "__main__":
    main()
