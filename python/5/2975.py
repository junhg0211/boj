def main():
    while True:
        now, type_, delta = input().split()
        now, delta = int(now), int(delta)

        if now == delta == 0 and type_ == "W":
            break

        if type_ == "D":
            print(now + delta)
        elif now - delta < -200:
            print("Not allowed")
        else:
            print(now - delta)


if __name__ == "__main__":
    main()
