from datetime import datetime, timedelta


def main():
    hour, minute, seconds = map(int, input().split())

    time = datetime(2022, 1, 1, hour, minute, seconds) + timedelta(seconds=int(input()))
    print(time.hour, time.minute, time.second)


if __name__ == '__main__':
    main()
