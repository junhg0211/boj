class SortedList:
    def __init__(self):
        self.list = list()

    def append(self, value: int) -> bool:
        for i, list_value in enumerate(self.list):
            if list_value > value:
                self.list.insert(i, value)
                return
        self.list.append(value)

    def contains(self, value: int, start: int = 0, end = None) -> bool:
        end = len(self.list) if end is None else end

        if start >= end:
            return False

        sub_list = self.list[start:end]
        anchor_index = len(sub_list) // 2
        anchor = sub_list[anchor_index]
        if value > anchor:
            return self.contains(value, anchor_index+start+1, end)
        if value < anchor:
            return self.contains(value, start, anchor_index+start)
        return True

    def extend(self, other: 'SortedList'):
        insert_at = 0
        for value in other.list:
            while insert_at < len(self.list) and value > self.list[insert_at]:
                insert_at += 1
            self.list.insert(insert_at, value)


def get_list_of(country: int, lists: list[SortedList]) -> bool:
    for sorted_list in lists:
        if sorted_list.contains(country):
            return sorted_list


def main():
    for _ in range(int(input())):
        _, planes = map(int, input().split())
        lists: list[SortedList] = list()
        paths = 0
        for _ in range(planes):
            from_country, to_country = map(int, input().split())

            from_world = get_list_of(from_country, lists)
            to_world = get_list_of(to_country, lists)

            if from_world and to_world:
                if from_world == to_world:
                    continue

                from_world.extend(to_world)
                lists.remove(to_world)
            elif from_world and to_world is None:
                from_world.append(to_country)
            elif to_world and from_world is None:
                to_world.append(from_country)
            else:
                new_world = SortedList()
                new_world.append(from_country)
                new_world.append(to_country)
                lists.append(new_world)
            paths += 1

        print(paths)


if __name__ == '__main__':
    main()
