def main():
    count = int(input())

    board = [list(map(int, input().split())) for _ in range(count)]

    max_friends = 0
    max_friends_man = 0
    for i in range(count):
        friends = set()
        for j in range(5):
            for k in range(count):
                if k in friends:
                    continue
                if board[k][j] == board[i][j]:
                    friends.add(k)
        friends = len(friends)
        if friends > max_friends:
            max_friends = friends
            max_friends_man = i

    print(max_friends_man + 1)


if __name__ == "__main__":
    main()
