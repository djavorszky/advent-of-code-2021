def task1(input):
    """
    find the number of times depth measurements increase
    >>> task1([199,200,208,210,200,207,240,269,260,263])
    7
    """
    return sum([1 for x, y in zip(input, input[1:]) if int(y) > int(x)])


def task2(input):
    """
    find the number of times depth measurements increase in a sliding 3-wide window
    >>> task2([199,200,208,210,200,207,240,269,260,263])
    5
    """
    return sum([1 for x, y in zip(input, input[3:]) if int(y) > int(x)])


if __name__ == "__main__":
    with open("inputs/day01.txt") as input:
        lines = [line.rstrip() for line in input.readlines()]

        print("task1:", task1(lines))
        print("task2:", task2(lines))
