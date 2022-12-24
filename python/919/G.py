E = 0.5
VU = 1/12
VN = 0.06


def tick():
    variance = sum(map(lambda x: (x-E)**2, (float(input()) for _ in range(5000)))) / 5000

    if abs(variance - VU) > abs(variance - VN):
        print('B')
    else:
        print('A')


def main():
    for _ in range(100):
        tick()


if __name__ == '__main__':
    main()
