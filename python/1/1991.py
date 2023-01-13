from dataclasses import dataclass
from typing import Optional

nodes = dict()


def root_first(start):
    result = start
    if nodes[start][1] != '.':
        result += root_first(nodes[start][1])
    if nodes[start][2] != '.':
        result += root_first(nodes[start][2])
    return result


def left_first(start):
    result = ''
    if nodes[start][1] != '.':
        result += left_first(nodes[start][1])
    result += start
    if nodes[start][2] != '.':
        result += left_first(nodes[start][2])
    return result


def root_last(start):
    result = ''
    if nodes[start][1] != '.':
        result += root_last(nodes[start][1])
    if nodes[start][2] != '.':
        result += root_last(nodes[start][2])
    result += start
    return result


def main():
    node_count = int(input())

    for _ in range(node_count):
        value, left, right = input().split()
        nodes[value] = (value, left, right)

    print(root_first('A'))
    print(left_first('A'))
    print(root_last('A'))


if __name__ == '__main__':
    main()
