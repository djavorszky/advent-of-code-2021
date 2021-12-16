def task1(input):
    """
    find the location of the submarine after a given list of commands
    >>> task1(["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"])
    150
    """
    vertical = 0
    horizontal = 0

    for line in input:
        cmd, val = line.split()
        val = int(val)
        if cmd == "forward":
            horizontal += val
        else:
            vertical += val if cmd == "down" else -val

    return vertical * horizontal


def task2(input):
    """
    find the location of the submarine in an alternative way after a given list of commands
    >>> task2(["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"])
    900
    """
    horizontal = 0
    aim = 0
    depth = 0

    for line in input:
        cmd, val = line.split()
        val = int(val)
        if cmd == "forward":
            horizontal += val
            depth += aim * val
        else:
            aim += val if cmd == "down" else -val

    return depth * horizontal


if __name__ == "__main__":
    with open("inputs/day02.txt") as input:
        lines = [line.rstrip() for line in input.readlines()]

        print("task1:", task1(lines))
        print("task2:", task2(lines))
