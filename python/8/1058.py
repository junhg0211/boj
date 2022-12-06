def main():
    friendss = dict()
    count = int(input())
    for i in range(count):
        friendss[i] = list()
        for j, is_friend in enumerate(input()):
            if is_friend == 'Y':
                friendss[i].append(j)

    max_inssa = 0
    for person, friends in friendss.items():
        inssa = 0
        friend_friends = list()
        for friend in friends:
            friend_friends.extend(friendss[friend])
        for i in range(count):
            if i == person:
                continue
            if i in friends or i in friend_friends:
                inssa += 1
        
        if inssa > max_inssa:
            max_inssa = inssa

    print(max_inssa)


if __name__ == '__main__':
    main()
