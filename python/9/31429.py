leaderboard = [
    (12, 1600),
    (11, 894),
    (11, 1327),
    (10, 1311),
    (9, 1004),
    (9, 1178),
    (9, 1357),
    (8, 837),
    (7, 1055),
    (6, 556),
    (6, 773),
]


def main():
    index = int(input()) - 1

    print(leaderboard[index][0], leaderboard[index][1])


if __name__ == "__main__":
    main()
