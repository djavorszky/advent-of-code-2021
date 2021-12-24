def task1(input):
    """
    find the location of the submarine after a given list of commands
    >>> task1(["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"])
    198
    """

    total_bits = [int(c) for c in input[0]]
    for line in input[1:]:
        bits = [int(c) for c in line]

        for idx, i in enumerate(bits):
            if i == 1:
                total_bits[idx] += 1

    gamma = int("".join(["1" if i >= len(input) / 2 else "0" for i in total_bits]), 2)
    eps = int("".join(["0" if i >= len(input) / 2 else "1" for i in total_bits]), 2)

    return gamma * eps


def most_common_bit(input, loc):
    """
    Returns the most common bit at location loc, or `None` if they are equal
    >>> most_common_bit(["100", "101", "110", "001"], 0)
    '1'
    >>> most_common_bit(["100", "101", "110", "001"], 1)
    '0'
    >>> most_common_bit(["100", "101", "110", "001"], 2)

    """
    one_count = 0
    for line in input:
        if line[loc] == "1":
            one_count += 1

    half = len(input) / 2

    if one_count == half:
        return None

    return "1" if one_count > half else "0"


def task2(input):
    """
    find the location of the submarine in an alternative way after a given list of commands
    >>> task2(["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"])
    230
    """

    co2_input = input.copy()
    o2_input = input

    bit_length = len(input[0])

    for i in range(bit_length):
        if len(o2_input) == 1:
            break

        mcb = most_common_bit(o2_input, i)
        mcb = mcb if mcb is not None else "1"

        o2_input = list(filter(lambda item: item[i] == mcb, o2_input))

    for i in range(bit_length):
        if len(co2_input) == 1:
            break

        mcb = most_common_bit(co2_input, i)
        mcb = mcb if mcb is not None else "1"

        co2_input = list(filter(lambda item: item[i] != mcb, co2_input))

    return int(o2_input[0], 2) * int(co2_input[0], 2)


if __name__ == "__main__":
    with open("inputs/day03.txt") as input:
        lines = [line.rstrip() for line in input.readlines()]

        print("task1:", task1(lines))
        print("task2:", task2(lines))
