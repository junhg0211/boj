def main():
    members = set()

    log_count = int(input())
    for _ in range(log_count):
        member, type_ = input().split()

        if type_ == 'enter':
            members.add(member)
        else:
            members.remove(member)

    print('\n'.join(sorted(members, reverse=True)))


if __name__ == '__main__':
    main()
