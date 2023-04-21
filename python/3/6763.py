def main():
    limit = int(input())
    speed = int(input())

    diff = speed - limit

    if diff >= 31:
        fee = 500
    elif diff >= 21:
        fee = 270
    elif diff >= 1:
        fee = 100
    else:
        fee = 0

    if fee <= 0:
        print('Congratulations, you are within the speed limit!')
    else:
        print(f'You are speeding and your fine is ${fee}.')


if __name__ == '__main__':
    main()
