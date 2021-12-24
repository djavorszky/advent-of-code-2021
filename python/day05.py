from util import traverse, point2idx


def task1(vents, map_width=991):
    """
    find number of overlapping vents horizontal and vertical vents
    >>> with open("inputs/test/day05.txt") as file:
    ...     task1([line.rstrip() for line in file.readlines()], map_width=10)
    5
    """

    terrain = [0 for _ in range(map_width * map_width)]

    for vent in vents:
        (p1, p2) = parse(vent)

        if p1[0] != p2[0] and p1[1] != p2[1]:
            continue

        for (x, y) in traverse(p1, p2):
            terrain[point2idx(x, y, map_width)] += 1

    return len([n for n in terrain if n > 1])


def parse(vent):
    """
    Parses the input string into two tuples of numbers
    >>> parse("0,8 -> 3,9")
    ((0, 8), (3, 9))
    """
    splits = vent.split(" -> ")
    start = splits[0].split(",")
    end = splits[1].split(",")
    return (int(start[0]), int(start[1])), (int(end[0]), int(end[1]))


def task2(vents, map_width):
    """
    find number of overlapping horizontal, vertical, and diagonal vents
    >>> with open("inputs/test/day05.txt") as file:
    ...     task2([line.rstrip() for line in file.readlines()], map_width=10)
    12
    """

    terrain = [0 for _ in range(map_width * map_width)]

    for vent in vents:
        (p1, p2) = parse(vent)

        for (x, y) in traverse(p1, p2):
            terrain[point2idx(x, y, map_width)] += 1

    return len([n for n in terrain if n > 1])


if __name__ == "__main__":
    with open("inputs/day05.txt") as file:
        lines = [line.rstrip() for line in file.readlines()]

        print("task1:", task1(lines, map_width=991))
        print("task2:", task2(lines, map_width=991))
