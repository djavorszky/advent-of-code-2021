import math


def task1(terrain):
    """
    Figure out the sum of the low points on the terrain
    >>> with open("inputs/test/day09.txt") as file:
    ...     task1([line.rstrip() for line in file.readlines()])
    15
    """

    terrain = list(map(lambda t: [int(x) for x in t], terrain))

    return sum([terrain[y][x] + 1 for x, y in get_low_points(terrain)])


neighbours = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
]


def get_neighbours(terrain, x, y):
    for nx, ny in neighbours:
        px, py = x + nx, y + ny

        if px < 0 or py < 0:
            continue

        try:
            terrain[py][px]
        except IndexError:
            continue

        yield px, py


def get_neighbour_points(terrain, x, y):
    for nx, ny in get_neighbours(terrain, x, y):
        yield terrain[ny][nx]


def get_low_points(terrain):
    for y_idx, x_list in enumerate(terrain):
        for x_idx, point in enumerate(x_list):
            if point == 9:
                continue

            if all([point < n for n in get_neighbour_points(terrain, x_idx, y_idx)]):
                yield x_idx, y_idx


def task2(terrain):
    """
    Find the three largest basins
    >>> with open("inputs/test/day09.txt") as file:
    ...     task2([line.rstrip() for line in file.readlines()])
    1134
    """

    terrain = list(map(lambda t: [int(x) for x in t], terrain))

    low_points = get_low_points(terrain)

    basins = sorted([get_basin(terrain, point) for point in low_points], reverse=True)

    return math.prod(basins[:3])


def get_basin(terrain, point):
    def walk_basin(current, seen):

        if current in seen or terrain[current[1]][current[0]] == 9:
            return 0

        seen.add(current)

        adjacents = [n for n in get_neighbours(terrain, current[0], current[1]) if n not in seen]

        return 1 + sum([walk_basin(n, seen) for n in adjacents])

    walked = set()

    return walk_basin(point, walked)


if __name__ == "__main__":
    with open("inputs/day09.txt") as file:
        lines = [line.rstrip() for line in file.readlines()]

        print("task1:", task1(lines))
        print("task2:", task2(lines))
